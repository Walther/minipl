use crate::{span::StartEndSpan, visitors::Visitable};

use super::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub initializer: Option<Expression>,
    pub span: StartEndSpan,
}

impl Variable {
    pub fn new(name: &str, initializer: Option<Expression>, span: StartEndSpan) -> Self {
        Self {
            name: name.to_owned(),
            initializer,
            span,
        }
    }
}

impl From<Variable> for Visitable {
    fn from(val: Variable) -> Self {
        Visitable::Variable(val)
    }
}
