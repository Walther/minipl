use std::io::{self, Write};

use crate::{
    parsing::{Statement, Stmt, VarType, Variable},
    runtime::{Environment, Object},
    tokens::RawToken::{
        And, Bang, Equal, False, Less, Minus, Number, Plus, Slash, Star, Text, True,
    },
};

use super::Visitor;
use crate::parsing::*;
use crate::runtime::RuntimeError;

use miette::Result;
use tracing::debug;

#[derive(Debug, Default)]
/// [Interpreter] is a [Visitor] for interpreting i.e. evaluating the program
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
    pub fn eval(&mut self, statements: &[Statement]) -> Result<(), RuntimeError> {
        for statement in statements {
            let result = self.visit_statement(statement)?;
            debug!("Interpreted: {result:?}");
        }

        Ok(())
    }

    // TODO: cleanup

    //TODO: this should probably be private or deprecated
    /// Internal helper function: evaluates a single [Expr], a raw metadata-less version of [Expression]
    fn eval_expr(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
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

    // TODO: clean up the .expr. stuff

    fn visit_binary(&mut self, b: &Binary) -> Result<Object, RuntimeError> {
        let right = self.eval_expr(&b.right.expr)?;
        let left = self.eval_expr(&b.left.expr)?;
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
                (l_object, r_object) => {
                    return Err(RuntimeError::PlusTypeMismatch(
                        l_object.kind_to_string(),
                        r_object.kind_to_string(),
                        b.left.span.into(),
                        b.right.span.into(),
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
                (l_object, r_object) => {
                    return Err(RuntimeError::EqualTypeMismatch(
                        l_object.kind_to_string(),
                        r_object.kind_to_string(),
                        b.left.span.into(),
                        b.right.span.into(),
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
                (l_object, r_object) => {
                    return Err(RuntimeError::LessTypeMismatch(
                        l_object.kind_to_string(),
                        r_object.kind_to_string(),
                        b.left.span.into(),
                        b.right.span.into(),
                    ))
                }
            },
            _ => {
                return Err(RuntimeError::UnexpectedBinaryOperator(
                    format!("{:?}", b.operator.token),
                    b.operator.span.into(),
                ))
            }
        };
        Ok(result)
    }

    fn visit_grouping(&mut self, g: &Grouping) -> Result<Object, RuntimeError> {
        // Ignore the grouping; evaluate inner expression
        self.eval_expr(&g.expression.expr)
    }

    fn visit_literal(&mut self, l: &Literal) -> Result<Object, RuntimeError> {
        let result = match &l.value.token {
            Number(n) => Object::Number(*n),
            Text(t) => Object::Text(t.clone()),
            False => Object::Boolean(false),
            True => Object::Boolean(true),
            _ => {
                return Err(RuntimeError::UnexpectedLiteral(
                    format!("{:?}", l.value.token),
                    l.value.span.into(),
                ))
            }
        };
        Ok(result)
    }

    fn visit_logical(&mut self, l: &Logical) -> Result<Object, RuntimeError> {
        let right = self.eval_expr(&l.right.expr)?;
        let left = self.eval_expr(&l.left.expr)?;
        let tokentype = l.operator.tokentype();
        let result = match tokentype {
            And => Object::Boolean(left.as_bool()? && right.as_bool()?),
            _ => {
                return Err(RuntimeError::UnexpectedLogicalOperator(
                    format!("{:?}", l.operator.token),
                    l.operator.span.into(),
                ))
            }
        };
        Ok(result)
    }

    fn visit_unary(&mut self, u: &Unary) -> Result<Object, RuntimeError> {
        let right = self.eval_expr(&u.right.expr)?;
        let result = match u.operator.tokentype() {
            Minus => Object::Number(-right.as_numeric()?),
            Bang => Object::Boolean(!right.as_bool()?),
            _ => {
                return Err(RuntimeError::UnexpectedUnaryOperator(
                    format!("{:?}", u.operator.token),
                    u.operator.span.into(),
                ))
            }
        };
        Ok(result)
    }

    fn visit_variable_usage(&self, name: &str) -> Result<Object, RuntimeError> {
        self.environment.get(name)
    }

    /// Evaluates a variable assignment. Has side effects: stores the variable in the current interpreter's `environment`.
    fn visit_assign(&mut self, a: &Assign) -> Result<Object, RuntimeError> {
        let value = self.eval_expr(&a.value)?;
        self.environment.assign(&a.name, value, a.token.span)
    }

    /// Evaluates a variable declaration i.e. the initial definition of a variable. Has side effects: stores the variable in the current interpreter's `environment`.
    fn eval_variable_declaration(&mut self, v: &Variable) -> Result<Object, RuntimeError> {
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

impl Visitor<Object, RuntimeError> for Interpreter {
    fn visit_expression(&mut self, expression: &Expression) -> Result<Object, RuntimeError> {
        self.eval_expr(&expression.expr)
    }

    fn visit_statement(&mut self, statement: &Statement) -> Result<Object, RuntimeError> {
        let expr = match &statement.stmt {
            Stmt::Assert(e) => {
                let result = self.visit_expression(e)?;
                match result.as_bool() {
                    Ok(bool) => {
                        if bool {
                            return Ok(Object::Nothing);
                        }
                        return Err(RuntimeError::AssertionFailed(e.span.into()));
                    }
                    Err(_) => return Err(RuntimeError::AssertExprNotTruthy(e.span.into())),
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
                    .map_err(|_| RuntimeError::ReadLineFailed)?;
                let old = self.environment.get(name)?;
                let new = match old {
                    Object::Number(_) => Object::Number(
                        buffer
                            .trim()
                            .parse()
                            .map_err(|_| RuntimeError::ReadParseIntFailed)?,
                    ),
                    Object::Text(_) => Object::Text(buffer),
                    Object::Boolean(_) => Object::Boolean(
                        buffer
                            .trim()
                            .parse()
                            .map_err(|_| RuntimeError::ReadParseBoolFailed)?,
                    ),
                    Object::Nothing => {
                        return Err(RuntimeError::ReadNothing);
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
                    Err(_) => {
                        return Err(RuntimeError::ForStartNonNumeric(
                            start.to_string(),
                            f.left.span.into(),
                        ))
                    }
                };
                let end = self.visit_expression(&f.right)?;
                let end = match end.as_numeric() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(RuntimeError::ForEndNonNumeric(
                            end.to_string(),
                            f.right.span.into(),
                        ))
                    }
                };
                if start > end {
                    return Err(RuntimeError::ForEndLarger(
                        f.left.span.into(),
                        f.right.span.into(),
                    ));
                }
                for i in start..=end {
                    match self.environment.assign(&name, Object::Number(i), f.span) {
                        Ok(_) => (),
                        Err(_) => return Err(RuntimeError::ForBadAssignment(name, f.span.into())),
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
                .map_err(|_| RuntimeError::PrintCouldNotFlush)?;
        };

        Ok(result)
    }
}
