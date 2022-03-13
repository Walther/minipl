use miette::{miette, Result};
use std::collections::HashMap;

use crate::visitors::interpreter::Object;

/// Environment is a scoping storage for variables
#[derive(Debug, Default, Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: &str) -> Result<Object> {
        self.values
            .get(name)
            .cloned()
            .ok_or_else(|| return miette!(format!("Undefined variable: {name}")))
    }
}
