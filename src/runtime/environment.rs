use miette::Result;
use std::collections::HashMap;

use crate::runtime::Object;
use crate::span::StartEndSpan;

use super::RuntimeError;

/// Environment is a scoping storage for variables
#[derive(Debug, Default, Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    /// Declares a new variable with the given name and value into the [Environment].
    /// All variables must be declared before use, and each identifier may be declared once only.
    pub fn define(
        &mut self,
        name: &str,
        value: Object,
        span: StartEndSpan,
    ) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            return Err(RuntimeError::VariableReDeclaration(span.into()));
        }
        self.values.insert(name.to_owned(), value);
        Ok(())
    }

    /// Gets the value of the variable with the given name from the [Environment]
    pub fn get(&self, name: &str) -> Result<Object, RuntimeError> {
        self.values
            .get(name)
            .cloned()
            .ok_or_else(|| RuntimeError::VariableGetFailed(name.to_owned()))
    }

    /// Assigns a new value to an existing variable in the [Environment].
    pub fn assign(
        &mut self,
        name: &str,
        value: Object,
        span: StartEndSpan,
    ) -> Result<Object, RuntimeError> {
        let current = match self.values.get(name) {
            Some(v) => v,
            None => return Err(RuntimeError::VariableAssignToUndeclared(span.into())),
        };
        // TODO: more robust type checking, going via tostring and format is ugly
        if value.kind_to_string() != current.kind_to_string() {
            return Err(RuntimeError::VariableAssignTypeMismatch(
                current.kind_to_string(),
                value.kind_to_string(),
                span.into(),
                span.into(),
            ));
        }
        self.values.insert(name.to_owned(), value.clone());
        Ok(value)
    }
}
