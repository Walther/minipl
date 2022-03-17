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
    self, Assert, Assign, Bang, Bool, Colon, End, Equal, False, For, Int, Less, Minus, Number,
    ParenLeft, ParenRight, Plus, Print, Range, Read, Semicolon, Slash, Star, Text, True, Var,
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

    fn declaration(&mut self) -> Result<Statement, ParseError> {
        if let Some(next) = self.tokens.peek() {
            let tokentype = next.tokentype();
            if matches!(tokentype, Var) {
                // parse a variable declaration
                self.var_declaration()
            } else {
                // parse some other statement
                self.statement()
            }
        } else {
            Err(ParseError::OutOfTokens((0, 0).into()))
        }
    }

    fn var_declaration(&mut self) -> Result<Statement, ParseError> {
        // consume the var token
        let var = self
            .tokens
            .next()
            .ok_or_else(|| ParseError::OutOfTokens((0, 0).into()))?;

        // get identifier
        let identifier;
        if let Some(next) = self.tokens.next() {
            if let RawToken::Identifier(name) = &next.token {
                identifier = name.clone();
            } else {
                return Err(ParseError::ExpectedIdentifier(
                    format!("{:?}", next.token),
                    next.span.into(),
                ));
            };
        } else {
            return Err(ParseError::OutOfTokens((0, 0).into()));
        }

        // require type annotation colon
        if let Some(next) = self.tokens.next() {
            if !matches!(next.tokentype(), Colon) {
                return Err(ParseError::ExpectedTypeAnnotation(
                    format!("{:?}", next.token),
                    next.span.into(),
                ));
            }
        } else {
            return Err(ParseError::OutOfTokens((0, 0).into()));
        }
        // get type annotation
        let kind; // TODO: type checking!
        if let Some(next) = self.tokens.next() {
            match next.tokentype() {
                Bool => kind = VarType::Bool,
                Int => kind = VarType::Int,
                RawToken::String => kind = VarType::Text,
                _ => {
                    return Err(ParseError::ExpectedTypeAnnotation(
                        format!("{:?}", next.token),
                        next.span.into(),
                    ));
                }
            }
        } else {
            return Err(ParseError::OutOfTokens((0, 0).into()));
        }

        // optional assignment
        // TODO: simplify
        if let Some(next) = self.tokens.next() {
            if matches!(next.tokentype(), Assign) {
                // get initializer expression
                let initializer = self.expression()?;
                let span = StartEndSpan::new(var.span.start, initializer.span.end);
                // get semicolon
                if let Some(_token) = self
                    .tokens
                    .next_if(|token| matches!(token.tokentype(), Semicolon))
                {
                    return Ok(Statement::new(
                        Stmt::VariableDefinition(Variable::new(
                            &identifier,
                            kind,
                            Some(initializer),
                            span,
                        )),
                        span,
                    ));
                }
                // otherwise, missing semicolon
                return Err(ParseError::MissingSemicolon(span.into()));
            } else if matches!(next.tokentype(), Semicolon) {
                let span = StartEndSpan::new(var.span.start, next.span.end - 1);
                return Ok(Statement::new(
                    Stmt::VariableDefinition(Variable::new(&identifier, kind, None, span)),
                    span,
                ));
            } else if matches!(next.tokentype(), Equal) {
                // Help the user: if we find an Equal operator after the type initializer, the user probably meant to use Assign
                return Err(ParseError::ExpectedWalrus((next.span).into()));
            }
            // otherwise, missing semicolon
            let span = StartEndSpan::new(var.span.start, next.span.end - 1);
            return Err(ParseError::MissingSemicolon(span.into()));
        }
        Err(ParseError::OutOfTokens((0, 0).into()))
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        if let Some(next) = self.tokens.peek() {
            let tokentype = next.tokentype();
            if matches!(tokentype, For) {
                self.for_statement()
            } else if matches!(tokentype, Assert) {
                // consume the assert token
                self.tokens.next();
                self.assert_statement()
            } else if matches!(tokentype, Print) {
                // consume the print token
                self.tokens.next();
                self.print_statement()
            } else if matches!(tokentype, Read) {
                // consume the read token
                self.tokens.next();
                self.read_statement()
            } else {
                self.epxr_statement()
            }
        } else {
            Err(ParseError::NothingToParse((0, 0).into()))
        }
    }

    fn assert_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expression()?;
        if let Some(_token) = self
            .tokens
            .next_if(|token| matches!(token.tokentype(), Semicolon))
        {
            Ok(Statement::new(Stmt::Assert(expr.clone()), expr.span))
        } else {
            Err(ParseError::MissingSemicolon(expr.span.into()))
        }
    }

    fn for_statement(&mut self) -> Result<Statement, ParseError> {
        // consume the for token // TODO: remove unwrap
        let start = self.tokens.next().unwrap();
        // variable name literal
        let name;
        if let Some(next) = self.tokens.next() {
            let tokentype = next.tokentype();
            match tokentype {
                RawToken::Identifier(n) => name = n,
                _ => {
                    return Err(ParseError::ForMissingVariable(
                        format!("{:?}", next.token),
                        next.span.into(),
                    ))
                }
            };
        } else {
            return Err(ParseError::NothingToParse((0, 0).into()));
        };
        // in keyword
        if let Some(next) = self.tokens.next() {
            let tokentype = next.tokentype();
            match tokentype {
                RawToken::In => (),
                _ => {
                    return Err(ParseError::ForMissingIn(
                        format!("{:?}", next.token),
                        next.span.into(),
                    ))
                }
            };
        } else {
            return Err(ParseError::NothingToParse((0, 0).into()));
        };
        // left expr
        let left = self.expression()?;
        // range literal
        if let Some(next) = self.tokens.next() {
            let tokentype = next.tokentype();
            match tokentype {
                Range => (),
                _ => {
                    return Err(ParseError::ForMissingRange(
                        format!("{:?}", next.token),
                        next.span.into(),
                    ))
                }
            };
        } else {
            return Err(ParseError::NothingToParse((0, 0).into()));
        };
        // right expr
        let right = self.expression()?;
        // do keyword
        if let Some(next) = self.tokens.next() {
            let tokentype = next.tokentype();
            match tokentype {
                RawToken::Do => (),
                _ => {
                    return Err(ParseError::ForMissingDo(
                        format!("{:?}", next.token),
                        next.span.into(),
                    ))
                }
            };
        } else {
            return Err(ParseError::NothingToParse((0, 0).into()));
        };
        // loop body
        let mut body = Vec::new();
        while let Some(next) = self.tokens.peek() {
            let tokentype = next.tokentype();
            // Have we found the end?
            if matches!(tokentype, End) {
                // consume the end token
                self.tokens.next();
                // expect to find for token
                if let Some(next) = self.tokens.next() {
                    let tokentype = next.tokentype();
                    match tokentype {
                        For => {
                            // expect to find semicolon
                            if let Some(_token) = self
                                .tokens
                                .next_if(|token| matches!(token.tokentype(), Semicolon))
                            {
                                break;
                            }
                            return Err(ParseError::MissingSemicolon(next.span.into()));
                        }
                        _ => {
                            return Err(ParseError::EndMissingFor(
                                format!("{:?}", next.token),
                                next.span.into(),
                            ))
                        }
                    }
                }
                return Err(ParseError::NothingToParse((0, 0).into()));
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
        let expr = self.expression()?;
        if let Some(_token) = self
            .tokens
            .next_if(|token| matches!(token.tokentype(), Semicolon))
        {
            Ok(Statement::new(Stmt::Print(expr.clone()), expr.span))
        } else {
            Err(ParseError::MissingSemicolon(expr.span.into()))
        }
    }

    fn read_statement(&mut self) -> Result<Statement, ParseError> {
        if let Some(token) = self.tokens.next() {
            let tokentype = token.tokentype();
            let name;
            let expr = match tokentype {
                RawToken::Identifier(n) => {
                    name = n.clone();
                    Expression::new(Expr::VariableUsage(n), token.span)
                }
                _ => {
                    return Err(ParseError::ReadToNonVariable(
                        format!("{:?}", token.token),
                        token.span.into(),
                    ))
                }
            };
            if let Some(_token) = self
                .tokens
                .next_if(|token| matches!(token.tokentype(), Semicolon))
            {
                Ok(Statement::new(Stmt::Read(name), token.span))
            } else {
                Err(ParseError::MissingSemicolon(expr.span.into()))
            }
        } else {
            Err(ParseError::OutOfTokens((0, 0).into()))
        }
    }

    fn epxr_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expression()?;
        if let Some(_token) = self
            .tokens
            .next_if(|token| matches!(token.tokentype(), Semicolon))
        {
            Ok(Statement::new(Stmt::Expression(expr.clone()), expr.span))
        } else {
            Err(ParseError::MissingSemicolon(expr.span.into()))
        }
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.and()?;
        let spanstart = expr.span.start;
        while let Some(next) = self.tokens.peek() {
            let tokentype = next.tokentype();
            // Assignment
            if matches!(tokentype, Assign) {
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
            let tokentype = next.tokentype();
            if matches!(tokentype, RawToken::And) {
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
            let tokentype = next.tokentype();
            if matches!(tokentype, Equal) {
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
            let tokentype = next.tokentype();
            if matches!(tokentype, Less) {
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
            let tokentype = next.tokentype();
            if matches!(tokentype, Minus | Plus) {
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
            let tokentype = next.tokentype();
            if matches!(tokentype, Slash | Star) {
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
        if let Some(next) = self.tokens.peek() {
            let spanstart = next.span.start;
            let tokentype = next.tokentype();
            if matches!(tokentype, Bang | Minus) {
                let operator = next.clone();
                self.tokens.next();
                let right = self.unary()?;
                return Ok(Expression::new(
                    Expr::Unary(Unary::new(operator, right.expr)),
                    StartEndSpan::new(spanstart, right.span.end),
                ));
            }
        } else {
            return Err(ParseError::NothingToParse((0, 0).into()));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        // At a terminal value, we need to always consume the token ?
        // TODO: verify
        if let Some(token) = self.tokens.next() {
            let tokentype = token.tokentype();
            match tokentype {
                False | True | Number(_) | Text(_) => Ok(Expression::new(
                    Expr::Literal(Literal::new(token.clone())),
                    token.span,
                )),
                RawToken::Identifier(name) => {
                    Ok(Expression::new(Expr::VariableUsage(name), token.span))
                }
                ParenLeft => {
                    let expr = self.expression()?;
                    if let Some(_token) =
                        self.tokens.next_if(|token| token.tokentype() == ParenRight)
                    {
                        Ok(Expression::new(
                            Expr::Grouping(Grouping::new(expr.expr)),
                            expr.span,
                        ))
                    } else {
                        Err(ParseError::MissingParen(token.span.into()))
                    }
                }
                _ => Err(ParseError::ExpectedExpression(
                    format!("{:?}", token.token),
                    token.span.into(),
                )),
            }
        } else {
            Err(ParseError::OutOfTokens((0, 0).into()))
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
