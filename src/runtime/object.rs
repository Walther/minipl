use std::fmt::Display;

use miette::{miette, Result};

use crate::{
    parsing::{
        expression::Expression,
        statement::{Statement, Stmt},
    },
    visitors::{Interpreter, Visitor},
};

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
    pub fn as_numeric(&self) -> Result<i64> {
        match self {
            Object::Number(n) => Ok(*n),
            _ => Err(miette!("Expected a numeric value, got: {:?}", self)),
        }
    }

    /// Fallible cast of an [Object] to a [bool].
    pub fn as_bool(&self) -> Result<bool> {
        match self {
            Object::Boolean(b) => Ok(*b),
            _ => Err(miette!("Expected a boolean value, got: {:?}", self)),
        }
    }

    /// Fallible cast of an [Object] to a [String].
    pub fn as_text(&self) -> Result<String> {
        match self {
            Object::Text(s) => Ok(s.to_string()),
            _ => Err(miette!("Expected a text value, got: {:?}", self)),
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

impl Visitor<Object> for Interpreter {
    fn visit_expression(&mut self, expression: &Expression) -> Result<Object> {
        self.eval_expr(&expression.expr)
    }

    fn visit_statement(&mut self, statement: &Statement) -> Result<Object> {
        let expr = match &statement.stmt {
            Stmt::Expression(expr) => expr,
            Stmt::Print(expr) => expr,
            Stmt::VariableDefinition(v) => {
                // TODO: what to do here?
                self.eval_variable_declaration(v)?;
                return Ok(Object::Nothing);
            }
        };
        let result = self.eval_expr(&expr.expr)?;
        if let Stmt::Print(_expr) = &statement.stmt {
            println!("{}", result)
        };

        Ok(result)
    }
}
