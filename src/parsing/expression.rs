#![allow(missing_docs)] // TODO: document

use crate::{span::StartEndSpan, tokens::Token};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Operator(Operator),
    Unary(Unary),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub expr: Expr,
    pub span: StartEndSpan,
}

impl Expression {
    pub fn new(expr: Expr, span: StartEndSpan) -> Self {
        Self { expr, span }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
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

#[derive(Clone, Debug, PartialEq)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Literal {
    pub value: Token, // TODO: or something even more literal?
}

impl Literal {
    pub fn new(value: Token) -> Self {
        Self { value }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operator {
    pub operator: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}
