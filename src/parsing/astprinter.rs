use super::expression::*;
use super::visitor::Visitor;

// TODO: this feels incredibly un-ergonomic, how is this visitor pattern best used in rust?

#[derive(Debug)]
pub struct ASTPrinter;

impl ASTPrinter {
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
        let mut string = format!("({}", name);
        for expr in exprs {
            string.push(' ');
            string.push_str(&self.print(&expr));
        }
        string.push(')');

        string
    }
}

impl Visitor<String> for ASTPrinter {
    // TODO: so unergonomic, so many clones...

    fn visit_binary(&mut self, b: &Binary) -> String {
        let exprs = vec![*b.left.clone(), *b.right.clone()].into_iter();
        self.parenthesize(format!("{:?}", b.operator.token).as_str(), exprs)
    }

    fn visit_grouping(&mut self, g: &Grouping) -> String {
        let exprs = vec![Expr::Grouping(g.clone())].into_iter();
        self.parenthesize("group", exprs)
    }

    fn visit_literal(&mut self, l: &Literal) -> String {
        format!("{:?}", l.value.token)
    }

    fn visit_operator(&mut self, o: &Operator) -> String {
        // TODO: is this actually even used anywhere?
        format!("{:?}", o.operator.token)
    }

    fn visit_unary(&mut self, u: &Unary) -> String {
        let exprs = vec![*u.right.clone()].into_iter();
        self.parenthesize(format!("{:?}", u.operator).as_str(), exprs)
    }
}
