use crate::span::StartEndSpan;

use super::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Bool,
    Int,
    Text,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub kind: VarType,
    pub initializer: Option<Expression>,
    pub span: StartEndSpan,
}

impl Variable {
    pub fn new(
        name: &str,
        kind: VarType,
        initializer: Option<Expression>,
        span: StartEndSpan,
    ) -> Self {
        Self {
            name: name.to_owned(),
            kind,
            initializer,
            span,
        }
    }
}
