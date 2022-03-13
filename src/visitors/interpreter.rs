use crate::{
    parsing::statement::{Statement, Stmt},
    tokens::RawToken::{Bang, Equal, False, Less, Minus, Number, Plus, Slash, Star, Text, True},
};

use super::{Visitable, Visitor};
use crate::parsing::expression::*;

use miette::{miette, Error, Result};

#[derive(Debug, Default)]
/// [Interpreter] is a [Visitor] for interpreting i.e. evaluating the given expression
pub struct Interpreter;

impl Interpreter {
    /// Creates a new [Interpreter] object
    pub fn new() -> Self {
        Self {}
    }
}

impl Interpreter {
    /// The primary function of the [Interpreter]: returns the evaluated [Object] value of a given expression
    pub fn eval(&mut self, visitable: &Visitable) -> Result<Object> {
        match visitable {
            Visitable::Expression(e) => self.visit_expression(e),
            Visitable::Statement(s) => self.visit_statement(s),
            Visitable::Variable(v) => self.visit_variable(v),
        }
    }

    /// The primary function of the [Interpreter]: evaluates all statements
    pub fn eval_all(&mut self, statements: &[Statement]) -> Result<(), Error> {
        for statement in statements {
            let expr = match &statement.stmt {
                Stmt::Expression(expr) => expr,
                Stmt::Print(expr) => expr,
            };
            let result = match &expr.expr {
                Expr::Binary(b) => self.visit_binary(b),
                Expr::Grouping(g) => self.visit_grouping(g),
                Expr::Literal(l) => self.visit_literal(l),
                Expr::Operator(_o) => panic!("Attempted to print a bare `Operator`. We should not have those left at parsing stage."),
                Expr::Unary(u) => self.visit_unary(u),
            }?;

            if let Stmt::Print(_expr) = &statement.stmt {
                println!("{:?}", result)
            }
        }

        Ok(())
    }

    // TODO: cleanup

    fn eval_expr(&mut self, expr: &Expr) -> Result<Object> {
        match expr {
            Expr::Binary(b) => self.visit_binary(b),
            Expr::Grouping(g) => self.visit_grouping(g),
            Expr::Literal(l) => self.visit_literal(l),
            Expr::Operator(_o) => panic!("Attempted to print a bare `Operator`. We should not have those left at parsing stage."),
            Expr::Unary(u) => self.visit_unary(u),
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
}

// TODO: does this make any sense whatsoever?

#[derive(Debug, Clone)]
/// The main enum of the runtime values within the language interpretation process
pub enum Object {
    /// Number value
    Number(i64),
    /// Text value
    Text(String),
    /// Boolean value
    Boolean(bool),
}

impl Object {
    /// Fallible cast of an [Object] to an [i64].
    pub fn as_numeric(&self) -> Result<i64, Error> {
        match self {
            Object::Number(n) => Ok(*n),
            _ => Err(miette!("Expected a numeric value, got: {:?}", self)),
        }
    }

    /// Fallible cast of an [Object] to a [bool].
    pub fn as_bool(&self) -> Result<bool, Error> {
        match self {
            Object::Boolean(b) => Ok(*b),
            _ => Err(miette!("Expected a boolean value, got: {:?}", self)),
        }
    }

    /// Fallible cast of an [Object] to a [String].
    pub fn as_text(&self) -> Result<String, Error> {
        match self {
            Object::Text(s) => Ok(s.to_string()),
            _ => Err(miette!("Expected a text value, got: {:?}", self)),
        }
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_expression(&mut self, expression: &Expression) -> Result<Object> {
        let expr = expression.expr.clone();
        match &expr {
            Expr::Binary(b) => self.visit_binary(b),
            Expr::Grouping(g) => self.visit_grouping(g),
            Expr::Literal(l) => self.visit_literal(l),
            Expr::Operator(_o) => Err(miette!("Attempted to print a bare `Operator`. We should not have those left at parsing stage.")),
            Expr::Unary(u) => self.visit_unary(u),
        }
    }

    fn visit_statement(&mut self, _statement: &Statement) -> Result<Object> {
        todo!()
    }

    fn visit_variable(&mut self, _variable: &crate::parsing::variable::Variable) -> Result<Object> {
        todo!()
    }
}
