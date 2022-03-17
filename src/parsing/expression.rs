#![allow(missing_docs)] // TODO: document

use crate::{span::StartEndSpan, tokens::Token};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Unary(Unary),
    VariableUsage(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub expr: Expr,
    pub span: StartEndSpan,
}

impl Expression {
    #[must_use]
    pub fn new(expr: Expr, span: StartEndSpan) -> Self {
        Self { expr, span }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assign {
    pub name: String,
    pub token: Token,
    pub value: Box<Expr>,
}

impl Assign {
    #[must_use]
    pub fn new(name: &str, token: Token, value: Expr) -> Self {
        Self {
            name: name.to_owned(),
            token,
            value: Box::new(value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn new(value: Token) -> Self {
        Self { value }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Logical {
    #[must_use]
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    #[must_use]
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}
