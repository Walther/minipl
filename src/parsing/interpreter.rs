use super::expression::*;
use super::visitor::Visitor;

pub struct Interpreter;

// TODO: do we need both separately, or one generic impl?

impl Visitor<i64> for Interpreter {
    fn visit_binary(&mut self, b: &Binary) -> i64 {
        todo!()
    }

    fn visit_grouping(&mut self, g: &Grouping) -> i64 {
        todo!()
    }

    fn visit_literal(&mut self, l: &Literal) -> i64 {
        todo!()
    }

    fn visit_operator(&mut self, o: &Operator) -> i64 {
        todo!()
    }

    fn visit_unary(&mut self, u: &Unary) -> i64 {
        todo!()
    }
}

impl Visitor<String> for Interpreter {
    fn visit_binary(&mut self, b: &Binary) -> String {
        todo!()
    }

    fn visit_grouping(&mut self, g: &Grouping) -> String {
        todo!()
    }

    fn visit_literal(&mut self, l: &Literal) -> String {
        todo!()
    }

    fn visit_operator(&mut self, o: &Operator) -> String {
        todo!()
    }

    fn visit_unary(&mut self, u: &Unary) -> String {
        todo!()
    }
}
