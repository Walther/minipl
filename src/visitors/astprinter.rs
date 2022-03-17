use miette::Result;

use crate::parsing::{
    statement::{Statement, Stmt},
    variable::Variable,
};

use super::Visitor;
use crate::parsing::expression::*;

const INDENT: &str = "    ";

// TODO: this feels incredibly un-ergonomic, how is this visitor pattern best used in rust?

#[derive(Debug)]
/// [`ASTPrinter`] is a [`Visitor`] for prettyprinting the abstract syntax tree of the given expression
pub struct ASTPrinter {
    nest_level: u64,
}

impl ASTPrinter {
    /// Creates a new [`ASTPrinter`] with `nest_level: 0`.
    #[must_use]
    pub fn default() -> Self {
        Self { nest_level: 0 }
    }

    /// The primary function of the [`ASTPrinter`]: returns the prettyprinted [String] representation of the abstract syntax tree of the given expression
    pub fn print(&mut self, statement: &Statement) -> Result<String> {
        self.visit_statement(statement)
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
            string.push_str(&self.visit_expr(&expr)?);
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

    fn indented_print(&mut self, value: impl std::fmt::Debug) -> String {
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
            Expr::Assign(a) => self.visit_assign(a),
            Expr::Binary(b) => self.visit_binary(b),
            Expr::Grouping(g) => self.visit_grouping(g),
            Expr::Literal(l) => self.visit_literal(l),
            Expr::Logical(l) => self.visit_logical(l),
            Expr::Unary(u) => self.visit_unary(u),
            Expr::VariableUsage(name) => Ok(format!("Variable usage, name: {name}")),
        }
    }

    fn visit_assign(&mut self, a: &Assign) -> Result<String> {
        let exprs = vec![*a.value.clone()].into_iter();
        self.nest_level += 1;
        let string = self.parenthesize_exprs(
            format!("Assign into variable, name: {:?}", a.name).as_str(),
            exprs,
        )?;
        self.nest_level -= 1;
        Ok(string)
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

    fn visit_logical(&mut self, l: &Logical) -> Result<String> {
        let exprs = vec![*l.left.clone(), *l.right.clone()].into_iter();
        self.nest_level += 1;
        let string =
            self.parenthesize_exprs(format!("Logical: {:?}", l.operator.token).as_str(), exprs)?;
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

    fn visit_variable_definition(&mut self, variable: &Variable) -> Result<String> {
        match &variable.initializer {
            Some(expression) => {
                let exprs = vec![expression.expr.clone()].into_iter();
                self.nest_level += 1;
                let string = self.parenthesize_exprs(
                    format!(
                        "Variable definition, name: {:?}, type: {:?}",
                        variable.name, variable.kind
                    )
                    .as_str(),
                    exprs,
                )?;
                self.nest_level -= 1;
                Ok(string)
            }
            // TODO: correct indentation
            None => Ok(format!(
                "Variable definition, name: {:?}, type: {:?}",
                variable.name, variable.kind
            )),
        }
    }
}

impl Visitor<String> for ASTPrinter {
    fn visit_expression(&mut self, expression: &Expression) -> Result<String> {
        self.visit_expr(&expression.expr)
    }

    fn visit_statement(&mut self, statement: &Statement) -> Result<String> {
        match &statement.stmt {
            Stmt::Assert(e) => Ok(format!("Assert: {}", self.visit_expression(e)?)),
            Stmt::Expression(e) => self.visit_expression(e),
            Stmt::Print(e) => {
                let exprs = vec![e.expr.clone()].into_iter();
                self.nest_level += 1;
                let string = self.parenthesize_exprs("Print", exprs)?;
                self.nest_level -= 1;
                Ok(string)
            }
            Stmt::Read(name) => Ok(format!("Read, into variable name: {name}")),
            Stmt::VariableDefinition(v) => self.visit_variable_definition(v),
            Stmt::Forloop(f) => {
                // TODO: better AST prettyprinting for for loops...
                let start = self.visit_expression(&f.left)?;
                let end = self.visit_expression(&f.right)?;
                let loop_start = format!(
                    "For loop start, variable name: {}, start: {:?}, end: {:?}",
                    f.variable,
                    start.trim(),
                    end.trim()
                );
                let mut loop_body = String::new();
                for statement in &f.body {
                    let str = self.visit_statement(statement)?;
                    loop_body.push_str(&str);
                    loop_body.push('\n');
                }
                let loop_end = "For loop end";
                Ok(format!("{loop_start} {loop_body} {loop_end}"))
            }
        }
    }
}
