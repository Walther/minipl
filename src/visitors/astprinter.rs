use anyhow::{Error, Result};

use crate::tokens::RawToken;

use super::Visitor;
use crate::parsing::expression::*;

const INDENT: &str = "    ";

// TODO: this feels incredibly un-ergonomic, how is this visitor pattern best used in rust?

#[derive(Debug)]
/// [ASTPrinter] is a [Visitor] for prettyprinting the abstract syntax tree of the given expression
pub struct ASTPrinter {
    nest_level: u64,
}

impl ASTPrinter {
    /// Creates a new [ASTPrinter] with `nest_level: 0`.
    pub fn default() -> Self {
        Self { nest_level: 0 }
    }

    /// The primary function of the [ASTPrinter]: returns the prettyprinted [String] representation of the abstract syntax tree of the given expression
    pub fn print(&mut self, expr: &Expr) -> Result<String, Error> {
        match expr {
            Expr::Binary(b) => self.visit_binary(b),
            Expr::Grouping(g) => self.visit_grouping(g),
            Expr::Literal(l) => self.visit_literal(l),
            Expr::Operator(_o) => panic!("Attempted to print a bare `Operator`. We should not have those left at parsing stage."),
            Expr::Unary(u) => self.visit_unary(u),
        }
    }

    fn parenthesize(
        &mut self,
        name: &str,
        exprs: impl Iterator<Item = Expr>,
    ) -> Result<String, Error> {
        let mut string = String::new();
        // TODO: less hacky indent tree
        if self.nest_level > 0 {
            string.push('\n');
            for _ in 0..self.nest_level {
                string.push_str(INDENT);
            }
        }

        string.push_str(format!("({}", name).as_str());
        for expr in exprs {
            string.push(' ');
            string.push_str(&self.print(&expr)?);
        }

        // TODO: less hacky indent tree
        if self.nest_level > 0 {
            string.push('\n');
            for _ in 0..self.nest_level {
                string.push_str(INDENT);
            }
        }
        string.push(')');

        Ok(string)
    }

    fn indented_print(&mut self, value: &RawToken) -> String {
        let mut string = String::new();
        // TODO: less hacky indent tree
        if self.nest_level > 0 {
            string.push('\n');
            for _ in 0..self.nest_level {
                string.push_str(INDENT);
            }
        }
        string.push_str(format!("{:?}", value).as_str());
        string
    }
}

impl Visitor<String, Error> for ASTPrinter {
    // TODO: so unergonomic, so many clones...

    fn visit_binary(&mut self, b: &Binary) -> Result<String, Error> {
        let exprs = vec![*b.left.clone(), *b.right.clone()].into_iter();
        self.nest_level += 1;
        let string = self.parenthesize(format!("{:?}", b.operator.token).as_str(), exprs)?;
        self.nest_level -= 1;
        Ok(string)
    }

    fn visit_grouping(&mut self, g: &Grouping) -> Result<String, Error> {
        let exprs = vec![Expr::Grouping(g.clone())].into_iter();
        self.nest_level += 1;
        let string = self.parenthesize("group", exprs)?;
        self.nest_level -= 1;
        Ok(string)
    }

    fn visit_literal(&mut self, l: &Literal) -> Result<String, Error> {
        self.nest_level += 1;
        let string = self.indented_print(&l.value.token);
        self.nest_level -= 1;
        Ok(string)
    }

    fn visit_unary(&mut self, u: &Unary) -> Result<String, Error> {
        let exprs = vec![*u.right.clone()].into_iter();
        self.nest_level += 1;
        let string = self.parenthesize(format!("{:?}", u.operator).as_str(), exprs)?;
        self.nest_level -= 1;
        Ok(string)
    }
}
