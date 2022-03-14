use miette::{miette, Diagnostic, Result, SourceSpan};
use std::collections::HashMap;
use thiserror::Error;

use crate::runtime::Object;
use crate::span::StartEndSpan;

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
    ) -> Result<(), EnvironmentError> {
        if self.values.contains_key(name) {
            return Err(EnvironmentError::ReDeclaration(span.into()));
        }
        self.values.insert(name.to_owned(), value);
        Ok(())
    }

    /// Gets the value of the variable with the given name from the [Environment]
    pub fn get(&self, name: &str) -> Result<Object> {
        self.values
            .get(name)
            .cloned()
            .ok_or_else(|| return miette!(format!("Undefined variable: {name}")))
    }

    /// Assigns a new value to an existing variable in the [Environment].
    pub fn assign(
        &mut self,
        name: &str,
        value: Object,
        span: StartEndSpan,
    ) -> Result<Object, EnvironmentError> {
        if !self.values.contains_key(name) {
            return Err(EnvironmentError::AssignToUndeclared(span.into()));
        }
        self.values.insert(name.to_owned(), value.clone());
        Ok(value)
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Variable error")]
#[diagnostic()]
/// Enum for the errors that can occur within the use of an [Environment]
pub enum EnvironmentError {
    #[diagnostic(help(
        "Try removing the latter `var` to reassign, or use a different identifier"
    ))]
    /// Attempted re-declaration of a variable. Each identifier may be declared once only
    ReDeclaration(#[label = "Attempted to re-declare existing variable name"] SourceSpan),
    #[diagnostic(help("Use the keyword `var` to declare the variable"))]
    /// Attempted to assign to an undeclared variable. All variables must be declared before use
    AssignToUndeclared(
        #[label = "Attempted to assign to a variable that has not been declared"] SourceSpan,
    ),
}
