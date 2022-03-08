use crate::tokens::RawToken::{False, Number, Text, True};

use super::expression::*;
use super::visitor::Visitor;

#[derive(Debug, Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn eval(&self, _ast: &Expr) -> Object {
        todo!()
    }
}

// TODO: does this make any sense whatsoever?

#[derive(Debug)]
pub enum Object {
    Number(i64),
    Text(String),
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
