use crate::tokens::RawToken::{False, Number, Text, True};

use super::Visitor;
use crate::parsing::expression::*;

#[derive(Debug, Default)]
/// [Interpreter] is a [Visitor] for interpreting i.e. evaluating the given expression
pub struct Interpreter;

impl Interpreter {
    /// The primary function of the [Interpreter]: returns the evaluated [Object] value of a given expression
    pub fn eval(&self, _ast: &Expr) -> Object {
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

impl Visitor<Object> for Interpreter {
    fn visit_binary(&mut self, _b: &Binary) -> Object {
        todo!()
    }

    fn visit_grouping(&mut self, _g: &Grouping) -> Object {
        todo!()
    }

    fn visit_literal(&mut self, l: &Literal) -> Object {
        match &l.value.token {
            Number(n) => Object::Number(*n),
            Text(t) => Object::Text(t.clone()),
            False => Object::Boolean(false),
            True => Object::Boolean(true),
            _ => {
                // TODO: error handling
                panic!("Unexpected literal");
            }
        }
    }

    fn visit_operator(&mut self, _o: &Operator) -> Object {
        todo!()
    }

    fn visit_unary(&mut self, _u: &Unary) -> Object {
        todo!()
    }
}
