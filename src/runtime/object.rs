use std::fmt::Display;

use miette::Result;

use crate::runtime::RuntimeError::{self, *};

#[derive(Debug, Clone)]
/// The main enum of the runtime values within the language interpretation process
pub enum Object {
    /// Number value
    Number(i64),
    /// Text value
    Text(String),
    /// Boolean value
    Boolean(bool),
    /// Empty value
    Nothing,
}

impl Object {
    /// Fallible cast of an [Object] to an [i64].
    pub fn as_numeric(&self) -> Result<i64, RuntimeError> {
        match self {
            Object::Number(n) => Ok(*n),
            _ => Err(AsNumericFailed(self.to_string())),
        }
    }

    /// Fallible cast of an [Object] to a [bool].
    pub fn as_bool(&self) -> Result<bool, RuntimeError> {
        match self {
            Object::Boolean(b) => Ok(*b),
            _ => Err(AsBooleanFailed(self.to_string())),
        }
    }

    /// Fallible cast of an [Object] to a [String].
    pub fn as_text(&self) -> Result<String, RuntimeError> {
        match self {
            Object::Text(s) => Ok(s.to_string()),
            _ => Err(AsTextFailed(self.to_string())),
        }
    }

    /// Returns the type of the object as a string. Used for diagnostic purposes
    pub fn kind_to_string(&self) -> String {
        match self {
            Object::Number(_) => "Number".to_string(),
            Object::Text(_) => "Text".to_string(),
            Object::Boolean(_) => "Boolean".to_string(),
            Object::Nothing => "Nothing".to_string(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(val) => write!(f, "{val}"),
            Object::Text(val) => write!(f, "{val}"),
            Object::Boolean(val) => write!(f, "{val}"),
            Object::Nothing => write!(f, "Nothing"),
        }
    }
}
