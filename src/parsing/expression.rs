#![allow(missing_docs)] // TODO: document

use crate::lexing::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
    Operator(Operator),
}

pub trait Expression: std::fmt::Debug + PartialEq {
    fn visit(&self);
}

impl Expression for Expr {
    fn visit(&self) {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    value: Token, // TODO: or something even more literal?
}

impl Literal {
    pub fn new(value: Token) -> Self {
        Self { value }
    }
}

// NOTE: all these Box<Expr> used to be Box<dyn Expression>. Not sure right now what the right choice would be

#[derive(Debug, PartialEq)]
pub struct Grouping {
    expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Unary {
    operator: Token,
    right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Operator {
    operator: Token,
}
