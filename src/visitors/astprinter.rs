use miette::{miette, Result};

use crate::{
    parsing::{
        statement::{Statement, Stmt},
        variable::Variable,
    },
    tokens::RawToken,
};

use super::{Visitable, Visitor};
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
    pub fn print(&mut self, visitable: &Visitable) -> Result<String> {
        match visitable {
            Visitable::Expression(e) => self.visit_expression(e),
            Visitable::Statement(s) => self.visit_statement(s),
            Visitable::Variable(v) => self.visit_variable(v),
        }
    }

    /// Silly helper for handling the recursion with the metadata-less internal types
    /// // TODO: cleanup
    fn print_expr(&mut self, e: &Expr) -> Result<String> {
        self.visit_expr(e)
    }

    fn parenthesize_exprs(
        &mut self,
        name: &str,
        exprs: impl Iterator<Item = Expr>,
    ) -> Result<String> {
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
            string.push_str(&self.print_expr(&expr)?);
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

    // TODO: cleanup

    fn visit_expr(&mut self, expr: &Expr) -> Result<String> {
        match &expr {
            Expr::Binary(b) => self.visit_binary(b),
            Expr::Grouping(g) => self.visit_grouping(g),
            Expr::Literal(l) => self.visit_literal(l),
            Expr::Operator(_o) => Err(miette!("Attempted to print a bare `Operator`. We should not have those left at parsing stage.")),
            Expr::Unary(u) => self.visit_unary(u),
            Expr::Variable(name) => Ok(name.to_string()),
        }
    }

    fn visit_binary(&mut self, b: &Binary) -> Result<String> {
        let exprs = vec![*b.left.clone(), *b.right.clone()].into_iter();
        self.nest_level += 1;
        let string = self.parenthesize_exprs(format!("{:?}", b.operator.token).as_str(), exprs)?;
        self.nest_level -= 1;
        Ok(string)
    }

    fn visit_grouping(&mut self, g: &Grouping) -> Result<String> {
        let exprs = vec![*g.expression.clone()].into_iter();
        self.nest_level += 1;
        let string = self.parenthesize_exprs("Group", exprs)?;
        self.nest_level -= 1;
        Ok(string)
    }

    fn visit_literal(&mut self, l: &Literal) -> Result<String> {
        self.nest_level += 1;
        let string = self.indented_print(&l.value.token);
        self.nest_level -= 1;
        Ok(string)
    }

    fn visit_unary(&mut self, u: &Unary) -> Result<String> {
        let exprs = vec![*u.right.clone()].into_iter();
        self.nest_level += 1;
        let string = self.parenthesize_exprs(format!("{:?}", u.operator).as_str(), exprs)?;
        self.nest_level -= 1;
        Ok(string)
    }
}

impl Visitor<String> for ASTPrinter {
    fn visit_expression(&mut self, expression: &Expression) -> Result<String> {
        let expr = expression.expr.clone();
        self.visit_expr(&expr)
    }

    fn visit_statement(&mut self, statement: &Statement) -> Result<String> {
        match &statement.stmt {
            Stmt::Expression(e) => self.visit_expression(e),
            Stmt::Print(e) => {
                let exprs = vec![e.expr.clone()].into_iter();
                self.nest_level += 1;
                let string = self.parenthesize_exprs("Print", exprs)?;
                self.nest_level -= 1;
                Ok(string)
            }
            Stmt::Variable(v) => self.visit_variable(v),
        }
    }

    fn visit_variable(&mut self, variable: &Variable) -> Result<String> {
        match &variable.initializer {
            Some(expression) => {
                let exprs = vec![expression.expr.clone()].into_iter();
                self.nest_level += 1;
                let string = self
                    .parenthesize_exprs(format!("Variable {:?}", variable.name).as_str(), exprs)?;
                self.nest_level -= 1;
                Ok(string)
            }
            // TODO: correct indentation
            None => Ok(format!("Variable {:?}", variable.name)),
        }
    }
}
