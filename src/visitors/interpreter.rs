use crate::{
    parsing::{Statement, Variable},
    runtime::{Environment, Object},
    tokens::RawToken::{Bang, Equal, False, Less, Minus, Number, Plus, Slash, Star, Text, True},
};

use super::Visitor;
use crate::parsing::expression::*;

use miette::{miette, Result};

#[derive(Debug, Default)]
/// [Interpreter] is a [Visitor] for interpreting i.e. evaluating the given expression
pub struct Interpreter {
    /// Environment for storing variables
    pub environment: Environment,
}

impl Interpreter {
    /// Creates a new [Interpreter] object
    pub fn new() -> Self {
        Self {
            environment: Default::default(),
        }
    }
}

impl Interpreter {
    /// The primary function of the [Interpreter]: evaluates all statements
    pub fn eval(&mut self, statements: &[Statement]) -> Result<()> {
        for statement in statements {
            self.visit_statement(statement)?;
        }

        Ok(())
    }

    // TODO: cleanup

    //TODO: this should probably be private or deprecated
    /// Internal helper function: evaluates a single [Expr], a raw metadata-less version of [Expression]
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Object> {
        match expr {
            Expr::Binary(b) => self.visit_binary(b),
            Expr::Grouping(g) => self.visit_grouping(g),
            Expr::Literal(l) => self.visit_literal(l),
            Expr::Operator(_o) => panic!("Attempted to print a bare `Operator`. We should not have those left at parsing stage."),
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

    fn visit_unary(&mut self, u: &Unary) -> Result<Object> {
        let right = self.eval_expr(&u.right)?;
        let result = match u.operator.tokentype() {
            Minus => Object::Number(-right.as_numeric()?),
            Bang => Object::Boolean(!right.as_bool()?),
            _ => return Err(miette!("Unexpected unary operator: {:?}", u.operator)),
        };
        Ok(result)
    }

    /// Evaluates a variable assignment. Has side effects: stores the variable in the current interpreter's `environment`.
    pub fn eval_variable_assignment(&mut self, v: &Variable) -> Result<Object> {
        if let Some(initializer) = &v.initializer {
            let value = self.eval_expr(&initializer.expr)?;
            self.environment.define(&v.name, value.clone(), v.span)?;
            Ok(value)
        } else {
            let value = Object::Nothing;
            self.environment.define(&v.name, value.clone(), v.span)?;
            Ok(value)
        }
    }
}
