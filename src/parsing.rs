#![allow(missing_docs)] // TODO: document

use miette::{Diagnostic, SourceSpan};
use std::iter::Peekable;
use thiserror::Error;

pub mod expression;
use expression::*;
pub mod statement;
use statement::*;
pub mod variable;

use crate::span::StartEndSpan;
use crate::tokens::RawToken::{
    self, Bang, Equal, False, Less, Minus, Number, ParenLeft, ParenRight, Plus, Print, Semicolon,
    Slash, Star, Text, True,
};
use crate::tokens::Token;

#[derive(Error, Debug, Diagnostic)]
#[error("Parse error")]
#[diagnostic()]
pub enum ParseError {
    NothingToParse(
        #[label = "Nothing to parse. Source contained ignorable tokens only."] SourceSpan,
    ),
    MissingParen(#[label = "Expected ( after this grouping"] SourceSpan),
    ExpectedExpressionFoundToken(
        String,
        #[label = "Expected expression, found token {0}"] SourceSpan,
    ),
    OutOfTokens(#[label = "Ran out of tokens while parsing"] SourceSpan),
    MissingSemicolon(#[label = "Expected ; after statement"] SourceSpan),
}

// Unstable syntax
// pub type Tokens = Peekable<impl Iterator<Item = Token>>;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Statement>, ParseError> {
    let mut tokens = tokens.iter().cloned().peekable();
    let mut statements: Vec<Statement> = Vec::new();
    while let Some(token) = tokens.peek() {
        // TODO: better handling
        if token.tokentype() == RawToken::EOF {
            tokens.next();
            break;
        }
        let stmt = statement(&mut tokens)?;
        statements.push(stmt);
    }

    Ok(statements)
}

pub fn statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    if let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Print) {
            // consume the print token
            tokens.next();
            print_statement(tokens)
        } else {
            epxr_statement(tokens)
        }
    } else {
        Err(ParseError::NothingToParse((0, 0).into()))
    }
}

pub fn print_statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    let expr = expression(tokens)?;
    if let Some(_token) = tokens.next_if(|token| matches!(token.tokentype(), Semicolon)) {
        Ok(Statement::new(Stmt::Print(expr)))
    } else {
        Err(ParseError::MissingSemicolon(expr.span.into()))
    }
}

pub fn epxr_statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    let expr = expression(tokens)?;
    if let Some(_token) = tokens.next_if(|token| matches!(token.tokentype(), Semicolon)) {
        Ok(Statement::new(Stmt::Expression(expr)))
    } else {
        Err(ParseError::MissingSemicolon(expr.span.into()))
    }
}

pub fn expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, ParseError> {
    equality(tokens)
}

pub fn equality(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, ParseError> {
    let mut expr = comparison(tokens)?;
    let spanstart = expr.span.start;
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Equal) {
            let operator = next.clone();
            tokens.next();
            let right = comparison(tokens)?;
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

pub fn comparison(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, ParseError> {
    let mut expr = term(tokens)?;
    let spanstart = expr.span.start;
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Less) {
            let operator = next.clone();
            tokens.next();
            let right = term(tokens)?;
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

pub fn term(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expression, ParseError> {
    let mut expr = factor(tokens)?;
    let spanstart = expr.span.start;
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Minus | Plus) {
            let operator = next.clone();
            tokens.next();
            let right = factor(tokens)?;
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

pub fn factor(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, ParseError> {
    let mut expr = unary(tokens)?;
    let spanstart = expr.span.start;
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Slash | Star) {
            let operator = next.clone();
            tokens.next();
            let right = unary(tokens)?;
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

pub fn unary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expression, ParseError> {
    if let Some(next) = tokens.peek() {
        let spanstart = next.span.start;
        let tokentype = next.tokentype();
        if matches!(tokentype, Bang | Minus) {
            let operator = next.clone();
            tokens.next();
            let right = unary(tokens)?;
            return Ok(Expression::new(
                Expr::Unary(Unary::new(operator, right.expr)),
                StartEndSpan::new(spanstart, right.span.end),
            ));
        }
    } else {
        return Err(ParseError::NothingToParse((0, 0).into()));
    }

    primary(tokens)
}

pub fn primary(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, ParseError> {
    // At a terminal value, we need to always consume the token ?
    // TODO: verify
    if let Some(token) = tokens.next() {
        let tokentype = token.tokentype();
        match tokentype {
            False | True | Number(_) | Text(_) => Ok(Expression::new(
                Expr::Literal(Literal::new(token.clone())),
                token.span,
            )),
            ParenLeft => {
                let expr = expression(tokens)?;
                if let Some(_token) = tokens.next_if(|token| token.tokentype() == ParenRight) {
                    Ok(Expression::new(
                        Expr::Grouping(Grouping::new(expr.expr)),
                        expr.span,
                    ))
                } else {
                    Err(ParseError::MissingParen(token.span.into()))
                }
            }
            _ => Err(ParseError::ExpectedExpressionFoundToken(
                format!("{:?}", token.token),
                token.span.into(),
            )),
        }
    } else {
        Err(ParseError::OutOfTokens((0, 0).into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::parsing::*;

    #[test]
    fn literal_one() {
        let one = Token::new(Number(1), StartEndSpan::new(0, 1));
        let semi = Token::new(Semicolon, StartEndSpan::new(1, 1));
        let parsed = parse(vec![one.clone(), semi]).unwrap();
        let expected = Statement::new(Stmt::Expression(Expression::new(
            Expr::Literal(Literal::new(one)),
            StartEndSpan::new(0, 1),
        )));
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

        let parsed = parse(tokens).unwrap();
        let expected = Statement::new(Stmt::Expression(Expression::new(
            Expr::Binary(Binary::new(
                Expr::Literal(Literal::new(one1)),
                equal,
                Expr::Literal(Literal::new(one2)),
            )),
            StartEndSpan::new(0, 3),
        )));
        assert_eq!(parsed[0], expected);
    }
}
