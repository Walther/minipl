use crate::tokens::RawToken::{Bang, False, Minus, Number, Text, True};

use super::Visitor;
use crate::parsing::expression::*;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, Default)]
/// [Interpreter] is a [Visitor] for interpreting i.e. evaluating the given expression
pub struct Interpreter;

impl Interpreter {
    /// The primary function of the [Interpreter]: returns the evaluated [Object] value of a given expression
    pub fn eval(&self, _ast: &Expr) -> Result<Object, Error> {
        todo!()
    }
}

// TODO: does this make any sense whatsoever?

#[derive(Debug)]
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
            _ => Err(anyhow!("Expected a numeric value, got: {:?}", self)),
        }
    }
}

impl Visitor<Object, Error> for Interpreter {
    fn visit_binary(&mut self, _b: &Binary) -> Result<Object, Error> {
        todo!()
    }

    fn visit_grouping(&mut self, g: &Grouping) -> Result<Object, Error> {
        self.eval(&g.expression)
    }

    fn visit_literal(&mut self, l: &Literal) -> Result<Object, Error> {
        let result = match &l.value.token {
            Number(n) => Object::Number(*n),
            Text(t) => Object::Text(t.clone()),
            False => Object::Boolean(false),
            True => Object::Boolean(true),
            _ => return Err(anyhow!("Unexpected literal: {:?}", l.value.token)),
        };
        Ok(result)
    }

    fn visit_unary(&mut self, u: &Unary) -> Result<Object, Error> {
        let right = self.eval(&u.right)?;
        let result = match u.operator.tokentype() {
            Minus => Object::Number(-right.as_numeric()?),
            Bang => todo!(),
            _ => return Err(anyhow!("Unexpected unary operator: {:?}", u.operator)),
        };
        Ok(result)
    }
}
