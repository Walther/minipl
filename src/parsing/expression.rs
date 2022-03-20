//! Expressions in the Mini-PL programming language.
//!
//! In the parsing phase, the source code is constructed into [Expression]s and [Statement](crate::parsing::Statement)s.

use crate::{span::StartEndSpan, tokens::Token};

#[derive(Clone, Debug, PartialEq)]
/// Low-level enum containing all possible expression variants.
pub enum Expr {
    /// Assignment to a variable
    Assign(Assign),
    /// Binary expression
    Binary(Binary),
    /// Grouping expression, mostly transparent
    Grouping(Grouping),
    /// Literal value expression
    Literal(Literal),
    /// Logical expression. Currently only AND exists
    Logical(Logical),
    /// Unary expression
    Unary(Unary),
    /// Usage of a variable
    VariableUsage(String),
}

#[derive(Clone, Debug, PartialEq)]
/// A richer [Expression] type that wraps the [`Expr`] type, and holds more metadata.
pub struct Expression {
    /// The contents of the [Expression], as a low-level [Expr]
    pub expr: Expr,
    /// The location span `(start, end)` of the [Expression]
    pub span: StartEndSpan,
}

impl Expression {
    #[must_use]
    /// Creates a new [Expression] given the low-level [Expr] and [StartEndSpan]
    pub fn new(expr: Expr, span: StartEndSpan) -> Self {
        Self { expr, span }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Assignment to a variable
pub struct Assign {
    /// Name of the variable
    pub name: String,
    /// The [Token] used in this assignment
    pub token: Token,
    /// The [Expr] to evaluate and then assign as the new value of the variable
    pub value: Box<Expr>,
}

impl Assign {
    #[must_use]
    /// Creates a new [Assign] [Expression]
    pub fn new(name: &str, token: Token, value: Expr) -> Self {
        Self {
            name: name.to_owned(),
            token,
            value: Box::new(value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Binary expression. Any expression that applies an operator between two sub-expressions.
pub struct Binary {
    /// Left hand side of the binary expression
    pub left: Box<Expression>,
    /// The operator of this binary expression
    pub operator: Token,
    /// Right hand side of the binary expression
    pub right: Box<Expression>,
}

impl Binary {
    #[must_use]
    /// Creates a new [Binary] [Expression]
    pub fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Grouping expression. These are constructed with the use of parens `()`.
pub struct Grouping {
    /// The expression contained within this grouping
    pub expression: Box<Expression>,
}

impl Grouping {
    #[must_use]
    /// Creates a new [Grouping] [Expression]
    pub fn new(expression: Expression) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Literal "expression". Contains a literal value.
pub struct Literal {
    /// The literal value
    pub value: Token, // TODO: or something even more literal?
}

impl Literal {
    #[must_use]
    /// Creates a new [Literal] [Expression]
    pub fn new(value: Token) -> Self {
        Self { value }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Logical operator expression. Currently, only logical AND `&` exists.
pub struct Logical {
    /// Left hand side of the logical expression
    pub left: Box<Expression>,
    /// The operator of this logical expression
    pub operator: Token,
    /// Right hand side of the logical expression
    pub right: Box<Expression>,
}

impl Logical {
    #[must_use]
    /// Creates a new [Logical] [Expression]
    pub fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Unary expression. Any expression that takes a unary operator and an expression.
pub struct Unary {
    /// The operator for this unary expression
    pub operator: Token,
    /// The expression to which to apply the unary operator
    pub right: Box<Expression>,
}

impl Unary {
    #[must_use]
    /// Creates a new [Unary] [Expression]
    pub fn new(operator: Token, right: Expression) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}
