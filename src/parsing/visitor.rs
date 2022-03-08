use super::expression::*;

// TODO: move this and decendents to a visitors/ folder

pub trait Visitor<T> {
    fn visit_binary(&mut self, b: &Binary) -> T;
    fn visit_grouping(&mut self, g: &Grouping) -> T;
    fn visit_literal(&mut self, l: &Literal) -> T;
    fn visit_operator(&mut self, o: &Operator) -> T;
    fn visit_unary(&mut self, u: &Unary) -> T;
}
