use crate::span::StartEndSpan;

use super::{Expression, Statement};

#[derive(Clone, Debug, PartialEq)]
pub struct Forloop {
    pub variable: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub body: Vec<Statement>,
    pub span: StartEndSpan,
}

impl Forloop {
    #[must_use]
    pub fn new(
        variable: &str,
        left: Expression,
        right: Expression,
        body: Vec<Statement>,
        span: StartEndSpan,
    ) -> Self {
        Self {
            variable: variable.to_owned(),
            left: Box::new(left),
            right: Box::new(right),
            body,
            span,
        }
    }
}
