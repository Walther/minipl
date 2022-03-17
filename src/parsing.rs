use std::iter::Peekable;
use std::vec::IntoIter;

pub mod expression;
pub use expression::*;
mod statement;
pub use statement::*;
mod variable;
pub use variable::*;
mod forloop;
pub use forloop::*;

use crate::span::StartEndSpan;
use crate::tokens::RawToken::{
    self, Assert, Assign, Bang, Bool, Colon, End, Equal, False, For, Identifier, Int, Less, Minus,
    Number, ParenLeft, ParenRight, Plus, Print, Range, Read, Semicolon, Slash, Star, Text, True,
    Var,
};
use crate::tokens::Token;
mod parse_error;
pub use parse_error::ParseError;

#[derive(Debug)]
/// The parser for the Mini-PL programming language
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    #[must_use]
    /// Initializes a parser with the given [`Vec`] of [`Token`]s
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Parses the tokens, returning [`Vec<Statement>`] or [`ParseError`]
    pub fn parse(&mut self) -> Result<Vec<Statement>, ParseError> {
        if self.tokens.len() == 0 {
            return Err(ParseError::NothingToParse((0, 0).into()));
        }

        let mut declarations: Vec<Statement> = Vec::new();
        while let Some(token) = self.tokens.peek() {
            // TODO: better handling
            if token.tokentype() == RawToken::EOF {
                self.tokens.next();
                break;
            }
            let declaration = self.declaration()?;
            declarations.push(declaration);
        }

        Ok(declarations)
    }

    /// Internal helper: returns the peeked next token, or an OutOfTokens error
    fn maybe_peek(&mut self) -> Result<&Token, ParseError> {
        if let Some(next) = self.tokens.peek() {
            Ok(next)
        } else {
            // TODO: proper error span!
            Err(ParseError::OutOfTokens((0, 0).into()))
        }
    }

    /// Internal helper: returns the next token, consumed from the iterator, or an OutOfTokens error
    fn maybe_next(&mut self) -> Result<Token, ParseError> {
        if let Some(next) = self.tokens.next() {
            Ok(next)
        } else {
            // TODO: proper error span!
            Err(ParseError::OutOfTokens((0, 0).into()))
        }
    }

    /// Internal helper: if the next token matches the given type, returns it and consumes it from the iterator. If the next one does not match, does not consume it and returns None
    fn next_if_tokentype(&mut self, tokentype: RawToken) -> Option<Token> {
        self.tokens.next_if(|token| token.tokentype() == tokentype)
    }

    /// Internal helper: expect the next token to be a Semicolon, and consume it, or return a MissingSemicolon error with the given span
    fn expect_semicolon(&mut self, span: StartEndSpan) -> Result<(), ParseError> {
        // get semicolon
        if self.next_if_tokentype(Semicolon).is_some() {
            return Ok(());
        }
        // otherwise, missing semicolon
        Err(ParseError::MissingSemicolon(span.into()))
    }

    fn declaration(&mut self) -> Result<Statement, ParseError> {
        let next = self.maybe_peek()?;
        match next.tokentype() {
            // parse a variable declaration
            Var => self.var_declaration(),
            // parse some other statement
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> Result<Statement, ParseError> {
        // consume the var token
        let var = self.maybe_next()?;

        // get identifier
        let next = self.maybe_next()?;
        let identifier = if let Identifier(name) = &next.token {
            name.clone()
        } else {
            return Err(ParseError::ExpectedIdentifier(
                format!("{:?}", next.token),
                next.span.into(),
            ));
        };

        // require type annotation colon
        let next = self.maybe_next()?;
        if !matches!(next.tokentype(), Colon) {
            return Err(ParseError::ExpectedTypeAnnotation(
                format!("{:?}", next.token),
                next.span.into(),
            ));
        }

        // get type annotation
        // TODO: parse time typechecking? or just at run time?
        let next = self.maybe_next()?;
        let kind = match next.tokentype() {
            Bool => VarType::Bool,
            Int => VarType::Int,
            RawToken::String => VarType::Text,
            _ => {
                return Err(ParseError::ExpectedTypeAnnotation(
                    format!("{:?}", next.token),
                    next.span.into(),
                ));
            }
        };

        // optional assignment
        let next = self.maybe_next()?;
        match next.tokentype() {
            Assign => {
                // get initializer expression
                let initializer = self.expression()?;
                let span = StartEndSpan::new(var.span.start, initializer.span.end);
                // get semicolon
                self.expect_semicolon(span)?;

                Ok(Statement::new(
                    Stmt::VariableDefinition(Variable::new(
                        &identifier,
                        kind,
                        Some(initializer),
                        span,
                    )),
                    span,
                ))
            }
            Semicolon => {
                let span = StartEndSpan::new(var.span.start, next.span.end - 1);
                Ok(Statement::new(
                    Stmt::VariableDefinition(Variable::new(&identifier, kind, None, span)),
                    span,
                ))
            }
            Equal => {
                // Help the user: if we find an Equal operator after the type initializer, the user probably meant to use Assign
                Err(ParseError::ExpectedWalrus((next.span).into()))
            }
            _ => {
                // otherwise, missing semicolon
                let span = StartEndSpan::new(var.span.start, next.span.end - 1);
                Err(ParseError::MissingSemicolon(span.into()))
            }
        }
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        let next = self.maybe_peek()?;

        match next.tokentype() {
            For => self.for_statement(),
            Assert => self.assert_statement(),
            Print => self.print_statement(),
            Read => self.read_statement(),
            _ => self.epxr_statement(),
        }
    }

    fn assert_statement(&mut self) -> Result<Statement, ParseError> {
        // consume the assert token
        let start = self.maybe_next()?;
        let expr = self.expression()?;
        let span = StartEndSpan::new(start.span.start, expr.span.end);
        self.expect_semicolon(span)?;
        Ok(Statement::new(Stmt::Assert(expr), span))
    }

    fn for_statement(&mut self) -> Result<Statement, ParseError> {
        // consume the for token
        let start = self.maybe_next()?;

        // variable name literal
        let next = self.maybe_next()?;
        let name = match next.tokentype() {
            Identifier(n) => n,
            _ => {
                return Err(ParseError::ForMissingVariable(
                    format!("{:?}", next.token),
                    next.span.into(),
                ))
            }
        };

        // in keyword
        let next = self.maybe_next()?;
        match next.tokentype() {
            RawToken::In => (),
            _ => {
                return Err(ParseError::ForMissingIn(
                    format!("{:?}", next.token),
                    next.span.into(),
                ))
            }
        };

        // left expr
        let left = self.expression()?;
        // range literal
        let next = self.maybe_next()?;
        match next.tokentype() {
            Range => (),
            _ => {
                return Err(ParseError::ForMissingRange(
                    format!("{:?}", next.token),
                    next.span.into(),
                ))
            }
        };

        // right expr
        let right = self.expression()?;
        // do keyword
        let next = self.maybe_next()?;
        match next.tokentype() {
            RawToken::Do => (),
            _ => {
                return Err(ParseError::ForMissingDo(
                    format!("{:?}", next.token),
                    next.span.into(),
                ))
            }
        };

        // loop body
        let mut body = Vec::new();
        while let Some(next) = self.tokens.peek() {
            // Have we found the end?
            if matches!(next.tokentype(), End) {
                // consume the end token
                self.tokens.next();
                // expect to find for token
                let next = self.maybe_next()?;
                match next.tokentype() {
                    For => {
                        // expect to find semicolon
                        self.expect_semicolon(next.span)?;
                        break;
                    }
                    _ => {
                        return Err(ParseError::EndMissingFor(
                            format!("{:?}", next.token),
                            next.span.into(),
                        ))
                    }
                }
            }
            // Otherwise, parse full statements into the loop body
            let statement = self.statement()?;
            body.push(statement);
        }

        let last = body.last().unwrap(); // TODO: remove unwrap
        let span = StartEndSpan::new(start.span.start, last.span.end);
        Ok(Statement::new(
            Stmt::Forloop(Forloop::new(&name, left, right, body, span)),
            span,
        ))
    }

    fn print_statement(&mut self) -> Result<Statement, ParseError> {
        // consume the print token
        // consume the assert token
        let start = self.maybe_next()?;
        let expr = self.expression()?;
        let span = StartEndSpan::new(start.span.start, expr.span.end);
        self.expect_semicolon(span)?;
        Ok(Statement::new(Stmt::Print(expr), span))
    }

    fn read_statement(&mut self) -> Result<Statement, ParseError> {
        // consume the read token
        let start = self.maybe_next()?;
        let next = self.maybe_next()?;
        let span = StartEndSpan::new(start.span.start, next.span.end);

        let name;
        let expr = match next.tokentype() {
            Identifier(n) => {
                name = n.clone();
                Expression::new(Expr::VariableUsage(n), span)
            }
            _ => {
                return Err(ParseError::ReadToNonVariable(
                    format!("{:?}", next.token),
                    span.into(),
                ))
            }
        };
        self.expect_semicolon(expr.span)?;
        Ok(Statement::new(Stmt::Read(name), span))
    }

    fn epxr_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expression()?;
        self.expect_semicolon(expr.span)?;
        Ok(Statement::new(Stmt::Expression(expr.clone()), expr.span))
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.and()?;
        let spanstart = expr.span.start;
        while let Some(next) = self.tokens.peek() {
            // Assignment
            if matches!(next.tokentype(), Assign) {
                let assign = next.clone();
                self.tokens.next();
                let right = self.and()?;
                // TODO: better name getter
                let name = match expr.expr {
                    Expr::VariableUsage(s) => s,
                    _ => {
                        return Err(ParseError::AssignToNonVariable(
                            format!("{:?}", expr.expr),
                            expr.span.into(),
                        ))
                    }
                };
                expr = Expression::new(
                    Expr::Assign(crate::parsing::expression::Assign::new(
                        &name, assign, right.expr,
                    )),
                    StartEndSpan::new(spanstart, right.span.end),
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;
        let spanstart = expr.span.start;
        while let Some(next) = self.tokens.peek() {
            if matches!(next.tokentype(), RawToken::And) {
                let operator = next.clone();
                self.tokens.next();
                let right = self.comparison()?;
                expr = Expression::new(
                    Expr::Logical(Logical::new(expr.expr, operator, right.expr)),
                    StartEndSpan::new(spanstart, right.span.end),
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;
        let spanstart = expr.span.start;
        while let Some(next) = self.tokens.peek() {
            if matches!(next.tokentype(), Equal) {
                let operator = next.clone();
                self.tokens.next();
                let right = self.comparison()?;
                expr = Expression::new(
                    Expr::Binary(Binary::new(expr.expr, operator, right.expr)),
                    StartEndSpan::new(spanstart, right.span.end),
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;
        let spanstart = expr.span.start;
        while let Some(next) = self.tokens.peek() {
            if matches!(next.tokentype(), Less) {
                let operator = next.clone();
                self.tokens.next();
                let right = self.term()?;
                expr = Expression::new(
                    Expr::Binary(Binary::new(expr.expr, operator, right.expr)),
                    StartEndSpan::new(spanstart, right.span.end),
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;
        let spanstart = expr.span.start;
        while let Some(next) = self.tokens.peek() {
            if matches!(next.tokentype(), Minus | Plus) {
                let operator = next.clone();
                self.tokens.next();
                let right = self.factor()?;
                expr = Expression::new(
                    Expr::Binary(Binary::new(expr.expr, operator, right.expr)),
                    StartEndSpan::new(spanstart, right.span.end),
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;
        let spanstart = expr.span.start;
        while let Some(next) = self.tokens.peek() {
            if matches!(next.tokentype(), Slash | Star) {
                let operator = next.clone();
                self.tokens.next();
                let right = self.unary()?;
                expr = Expression::new(
                    Expr::Binary(Binary::new(expr.expr, operator, right.expr)),
                    StartEndSpan::new(spanstart, right.span.end),
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        let next = self.maybe_peek()?;
        let spanstart = next.span.start;
        if matches!(next.tokentype(), Bang | Minus) {
            let operator = next.clone();
            self.tokens.next();
            let right = self.unary()?;
            return Ok(Expression::new(
                Expr::Unary(Unary::new(operator, right.expr)),
                StartEndSpan::new(spanstart, right.span.end),
            ));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        // At a terminal value, we need to always consume the token
        let next = self.maybe_next()?;
        match next.tokentype() {
            False | True | Number(_) | Text(_) => Ok(Expression::new(
                Expr::Literal(Literal::new(next.clone())),
                next.span,
            )),
            Identifier(name) => Ok(Expression::new(Expr::VariableUsage(name), next.span)),
            ParenLeft => {
                let expr = self.expression()?;
                if let Some(_token) = self.next_if_tokentype(ParenRight) {
                    Ok(Expression::new(
                        Expr::Grouping(Grouping::new(expr.expr)),
                        expr.span,
                    ))
                } else {
                    Err(ParseError::MissingParen(next.span.into()))
                }
            }
            _ => Err(ParseError::ExpectedExpression(
                format!("{:?}", next.token),
                next.span.into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parsing::*;

    #[test]
    fn literal_one() {
        let one = Token::new(Number(1), StartEndSpan::new(0, 1));
        let semi = Token::new(Semicolon, StartEndSpan::new(1, 1));
        let mut parser = Parser::new(vec![one.clone(), semi]);
        let parsed = parser.parse().unwrap();
        let span = StartEndSpan::new(0, 1);
        let expected = Statement::new(
            Stmt::Expression(Expression::new(Expr::Literal(Literal::new(one)), span)),
            span,
        );
        assert_eq!(parsed[0], expected);
    }

    #[test]
    fn one_equals_one() {
        // TODO: this is extremely unergonomic, fix
        let one1 = Token::new(Number(1), StartEndSpan::new(0, 1));
        let equal = Token::new(Equal, StartEndSpan::new(1, 2));
        let one2 = Token::new(Number(1), StartEndSpan::new(2, 3));
        let semi = Token::new(Semicolon, StartEndSpan::new(3, 4));

        let tokens = vec![one1.clone(), equal.clone(), one2.clone(), semi];
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let expected = Statement::new(
            Stmt::Expression(Expression::new(
                Expr::Binary(Binary::new(
                    Expr::Literal(Literal::new(one1)),
                    equal,
                    Expr::Literal(Literal::new(one2)),
                )),
                StartEndSpan::new(0, 3),
            )),
            StartEndSpan::new(0, 3),
        );
        assert_eq!(parsed[0], expected);
    }
}
