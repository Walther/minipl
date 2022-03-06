use crate::rawtoken::RawToken;

use super::expression::*;
use super::visitor::Visitor;

// TODO: this feels incredibly un-ergonomic, how is this visitor pattern best used in rust?

#[derive(Debug)]
pub struct ASTPrinter {
    nest_level: u64,
}

impl ASTPrinter {
    pub fn default() -> Self {
        Self { nest_level: 0 }
    }

    pub fn print(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(b) => self.visit_binary(b),
            Expr::Grouping(g) => self.visit_grouping(g),
            Expr::Literal(l) => self.visit_literal(l),
            Expr::Operator(o) => self.visit_operator(o),
            Expr::Unary(u) => self.visit_unary(u),
        }
    }

    fn parenthesize(&mut self, name: &str, exprs: impl Iterator<Item = Expr>) -> String {
        let mut string = String::new();
        // TODO: less hacky indent tree
        if self.nest_level > 0 {
            string.push('\n');
            for _ in 0..self.nest_level {
                string.push_str("  ");
            }
        }

        string.push_str(format!("({}", name).as_str());
        for expr in exprs {
            string.push(' ');
            string.push_str(&self.print(&expr));
        }

        // TODO: less hacky indent tree
        if self.nest_level > 0 {
            string.push('\n');
            for _ in 0..self.nest_level {
                string.push_str("  ");
            }
        }
        string.push(')');

        string
    }

    fn indented_print(&mut self, value: &RawToken) -> String {
        let mut string = String::new();
        // TODO: less hacky indent tree
        if self.nest_level > 0 {
            string.push('\n');
            for _ in 0..self.nest_level {
                string.push_str("  ");
            }
        }
        string.push_str(format!("{:?}", value).as_str());
        string
    }
}

impl Visitor<String> for ASTPrinter {
    // TODO: so unergonomic, so many clones...

    fn visit_binary(&mut self, b: &Binary) -> String {
        let exprs = vec![*b.left.clone(), *b.right.clone()].into_iter();
        self.nest_level += 1;
        let string = self.parenthesize(format!("{:?}", b.operator.token).as_str(), exprs);
        self.nest_level -= 1;
        string
    }

    fn visit_grouping(&mut self, g: &Grouping) -> String {
        let exprs = vec![Expr::Grouping(g.clone())].into_iter();
        self.nest_level += 1;
        let string = self.parenthesize("group", exprs);
        self.nest_level -= 1;
        string
    }

    fn visit_literal(&mut self, l: &Literal) -> String {
        self.nest_level += 1;
        let string = self.indented_print(&l.value.token);
        self.nest_level -= 1;
        string
    }

    fn visit_operator(&mut self, o: &Operator) -> String {
        // TODO: is this actually even used anywhere?
        self.nest_level += 1;
        let string = self.indented_print(&o.operator.token);
        self.nest_level -= 1;
        string
    }

    fn visit_unary(&mut self, u: &Unary) -> String {
        let exprs = vec![*u.right.clone()].into_iter();
        self.nest_level += 1;
        let string = self.parenthesize(format!("{:?}", u.operator).as_str(), exprs);
        self.nest_level -= 1;
        string
    }
}
