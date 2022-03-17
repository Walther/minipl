use std::io::{self, Write};

use crate::{
    parsing::{Statement, Stmt, VarType, Variable},
    runtime::{Environment, Object},
    tokens::RawToken::{
        And, Bang, Equal, False, Less, Minus, Number, Plus, Slash, Star, Text, True,
    },
};

use super::Visitor;
use crate::parsing::expression::*;

use miette::{miette, Result};
use tracing::debug;

#[derive(Debug, Default)]
/// [Interpreter] is a [Visitor] for interpreting i.e. evaluating the given expression
pub struct Interpreter {
    /// Environment for storing variables
    pub environment: Environment,
}

impl Interpreter {
    /// Creates a new [Interpreter] object
    #[must_use]
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
        }
    }
}

impl Interpreter {
    /// The primary function of the [Interpreter]: evaluates all statements
    pub fn eval(&mut self, statements: &[Statement]) -> Result<()> {
        for statement in statements {
            let result = self.visit_statement(statement)?;
            debug!("Interpreted: {result:?}");
        }

        Ok(())
    }

    // TODO: cleanup

    //TODO: this should probably be private or deprecated
    /// Internal helper function: evaluates a single [Expr], a raw metadata-less version of [Expression]
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Object> {
        match expr {
            Expr::Assign(a) => self.visit_assign(a),
            Expr::Binary(b) => self.visit_binary(b),
            Expr::Grouping(g) => self.visit_grouping(g),
            Expr::Literal(l) => self.visit_literal(l),
            Expr::Logical(l) => self.visit_logical(l),
            Expr::Unary(u) => self.visit_unary(u),
            Expr::VariableUsage(v) => self.visit_variable_usage(v),
        }
    }

    fn visit_binary(&mut self, b: &Binary) -> Result<Object> {
        let right = self.eval_expr(&b.right)?;
        let left = self.eval_expr(&b.left)?;
        let tokentype = b.operator.tokentype();
        let result = match tokentype {
            Minus => Object::Number(left.as_numeric()? - right.as_numeric()?),
            Slash => Object::Number(left.as_numeric()? / right.as_numeric()?),
            Star => Object::Number(left.as_numeric()? * right.as_numeric()?),
            Plus => match (&left, &right) {
                (Object::Number(_), Object::Number(_)) => {
                    Object::Number(left.as_numeric()? + right.as_numeric()?)
                }
                (Object::Text(_), Object::Text(_)) => {
                    Object::Text(format!("{}{}", left.as_text()?, right.as_text()?))
                }
                (_, _) => {
                    return Err(miette!(
                    "Plus operator can only be used for Number+Number or Text+Text, got: {:?} + {:?}",
                    left.clone(),
                    right.clone()
                ))
                }
            },
            Equal => match (&left, &right) {
                (Object::Number(_), Object::Number(_)) => {
                    Object::Boolean(left.as_numeric()? == right.as_numeric()?)
                }
                (Object::Text(_), Object::Text(_)) => {
                    Object::Boolean(left.as_text()? == right.as_text()?)
                }
                (_, _) => {
                    return Err(miette!(
                    "Equal operator can only be used for Number=Number or Text=Text, got: {:?} = {:?}",
                    left.clone(),
                    right.clone()
                ))
                }
            },
            Less => match (&left, &right) {
                (Object::Number(_), Object::Number(_)) => {
                    Object::Boolean(left.as_numeric()? < right.as_numeric()?)
                }
                (Object::Text(_), Object::Text(_)) => {
                    Object::Boolean(left.as_text()? < right.as_text()?)
                }
                (_, _) => {
                    return Err(miette!(
                    "Less operator can only be used for Number<Number or Text<Text, got: {:?} < {:?}",
                    left.clone(),
                    right.clone()
                ))
                }
            },
            _ => return Err(miette!("Unexpected unary operator: {:?}", b.operator)),
        };
        Ok(result)
    }

    fn visit_grouping(&mut self, g: &Grouping) -> Result<Object> {
        // Ignore the grouping; evaluate inner expression
        self.eval_expr(&g.expression)
    }

    fn visit_literal(&mut self, l: &Literal) -> Result<Object> {
        let result = match &l.value.token {
            Number(n) => Object::Number(*n),
            Text(t) => Object::Text(t.clone()),
            False => Object::Boolean(false),
            True => Object::Boolean(true),
            _ => return Err(miette!("Unexpected literal: {:?}", l.value.token)),
        };
        Ok(result)
    }

    fn visit_logical(&mut self, l: &Logical) -> Result<Object> {
        let right = self.eval_expr(&l.right)?;
        let left = self.eval_expr(&l.left)?;
        let tokentype = l.operator.tokentype();
        let result = match tokentype {
            And => Object::Boolean(left.as_bool()? && right.as_bool()?),
            _ => return Err(miette!("Unexpected logical operator: {:?}", l.operator)),
        };
        Ok(result)
    }

    fn visit_unary(&mut self, u: &Unary) -> Result<Object> {
        let right = self.eval_expr(&u.right)?;
        let result = match u.operator.tokentype() {
            Minus => Object::Number(-right.as_numeric()?),
            Bang => Object::Boolean(!right.as_bool()?),
            _ => return Err(miette!("Unexpected unary operator: {:?}", u.operator)),
        };
        Ok(result)
    }

    fn visit_variable_usage(&self, name: &str) -> Result<Object> {
        self.environment.get(name)
    }

    /// Evaluates a variable assignment. Has side effects: stores the variable in the current interpreter's `environment`.
    fn visit_assign(&mut self, a: &Assign) -> Result<Object> {
        let value = self.eval_expr(&a.value)?;
        match self.environment.assign(&a.name, value, a.token.span) {
            Ok(o) => Ok(o),
            Err(_e) => {
                // TODO: proper bubbling up of EnvironmentError
                return Err(miette!("Failed to assign variable: {:?}", a.name));
            }
        }
    }

    /// Evaluates a variable declaration i.e. the initial definition of a variable. Has side effects: stores the variable in the current interpreter's `environment`.
    pub fn eval_variable_declaration(&mut self, v: &Variable) -> Result<Object> {
        if let Some(initializer) = &v.initializer {
            let value = self.eval_expr(&initializer.expr)?;
            self.environment.define(&v.name, value.clone(), v.span)?;
            Ok(value)
        } else {
            let default_value = match v.kind {
                VarType::Bool => Object::Boolean(false),
                VarType::Int => Object::Number(0),
                VarType::Text => Object::Text("".to_owned()),
            };
            self.environment
                .define(&v.name, default_value.clone(), v.span)?;
            Ok(default_value)
        }
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_expression(&mut self, expression: &Expression) -> Result<Object> {
        self.eval_expr(&expression.expr)
    }

    fn visit_statement(&mut self, statement: &Statement) -> Result<Object> {
        let expr = match &statement.stmt {
            Stmt::Assert(e) => {
                let result = self.visit_expression(e)?;
                match result.as_bool() {
                    Ok(bool) => {
                        if bool {
                            return Ok(Object::Nothing);
                        }
                        return Err(miette!("Assertion failed: {:?}", e.expr));
                    }
                    Err(_) => return Err(miette!("Assert statement must evaluate to a boolean")),
                };
            }
            Stmt::Expression(expr) | Stmt::Print(expr) => expr,
            Stmt::Read(name) => {
                // TODO: better input handling
                // TODO: less hacky
                let mut buffer = String::new();
                let stdin = io::stdin();
                stdin
                    .read_line(&mut buffer)
                    .map_err(|_| return miette!("Failed to read a variable from stdin"))?;
                let old = self.environment.get(name)?;
                let new = match old {
                    Object::Number(_) => Object::Number(buffer.trim().parse().map_err(|_| {
                        return miette!("Failed to parse the read variable into an int");
                    })?),
                    Object::Text(_) => Object::Text(buffer),
                    Object::Boolean(_) => Object::Boolean(buffer.trim().parse().map_err(|_| {
                        return miette!("Failed to parse the read variable into a boolean");
                    })?),
                    Object::Nothing => {
                        return Err(miette!("Internal error, writing to a Nothing value"))
                    }
                };
                let object = self.environment.assign(name, new, statement.span)?;
                return Ok(object);
            }
            Stmt::VariableDefinition(v) => return self.eval_variable_declaration(v),
            Stmt::Forloop(f) => {
                let name = f.variable.clone();
                // NOTE: "The for control variable behaves like a constant inside the loop: it cannot be assigned another value (before exiting the for statement)"
                // This means we evaluate the start and end only once, based on the initial start..end declaration
                let start = self.visit_expression(&f.left)?;
                let start = match start.as_numeric() {
                    Ok(num) => num,
                    Err(_) => return Err(miette!("For loop start must be numeric")),
                };
                let end = self.visit_expression(&f.right)?;
                let end = match end.as_numeric() {
                    Ok(num) => num,
                    Err(_) => return Err(miette!("For loop end must be numeric")),
                };
                if start > end {
                    return Err(miette!("For loop end must be larger than start"));
                }
                for i in start..=end {
                    match self.environment.assign(&name, Object::Number(i), f.span) {
                        Ok(_) => (),
                        Err(_) => return Err(miette!("Assignment error during for loop")),
                    };
                    for statement in &f.body {
                        self.visit_statement(statement)?;
                    }
                }
                return Ok(Object::Nothing);
            }
        };
        let result = self.eval_expr(&expr.expr)?;
        if let Stmt::Print(_expr) = &statement.stmt {
            // NOTE: the course project spec is slightly unclear on whether a print statement should contain an implicit newline or not
            print!("{}", result);
            io::stdout()
                .flush()
                .map_err(|_| miette!("Runtime error: could not flush stdout after print"))?;
        };

        Ok(result)
    }
}
