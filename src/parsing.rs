#![allow(missing_docs)] // TODO: document

use miette::{Diagnostic, SourceSpan};
use std::iter::Peekable;
use thiserror::Error;

pub mod expression;
pub use expression::*;
pub mod statement;
pub use statement::*;
pub mod variable;
pub use variable::*;

use crate::span::StartEndSpan;
use crate::tokens::RawToken::{
    self, Assign, Bang, Bool, Colon, Equal, False, Int, Less, Minus, Number, ParenLeft, ParenRight,
    Plus, Print, Semicolon, Slash, Star, Text, True, Var,
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
    ExpectedExpression(
        String,
        #[label = "Expected expression, found token {0}"] SourceSpan,
    ),
    ExpectedIdentifier(
        String,
        #[label = "Expected identifier, found token {0}"] SourceSpan,
    ),
    ExpectedTypeAnnotation(
        String,
        #[label = "Expected identifier, found token {0}"] SourceSpan,
    ),
    ExpectedAssignment(
        String,
        #[label = "Expected assignment operator :=, found token {0}"] SourceSpan,
    ),
    #[diagnostic(help("Usage: variable_name := new_value"))]
    AssignToNonVariable(
        String,
        #[label = "Expected assignment to variable, found token {0}"] SourceSpan,
    ),
    #[diagnostic(help("Use the assignment operator := instead of = for declaring a variable"))]
    ExpectedWalrus(#[label = "Expected assignment operator `:=`, found `=`"] SourceSpan),
    OutOfTokens(#[label = "Ran out of tokens while parsing"] SourceSpan),
    MissingSemicolon(#[label = "Expected ; after statement"] SourceSpan),
}

// Unstable syntax
// pub type Tokens = Peekable<impl Iterator<Item = Token>>;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Statement>, ParseError> {
    let mut tokens = tokens.iter().cloned().peekable();
    if tokens.len() == 0 {
        return Err(ParseError::NothingToParse((0, 0).into()));
    }

    let mut declarations: Vec<Statement> = Vec::new();
    while let Some(token) = tokens.peek() {
        // TODO: better handling
        if token.tokentype() == RawToken::EOF {
            tokens.next();
            break;
        }
        let declaration = declaration(&mut tokens)?;
        declarations.push(declaration);
    }

    Ok(declarations)
}

pub fn declaration(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    if let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Var) {
            // parse a variable declaration
            var_declaration(tokens)
        } else {
            // parse some other statement
            statement(tokens)
        }
    } else {
        Err(ParseError::OutOfTokens((0, 0).into()))
    }
}

pub fn var_declaration(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    // consume the var token
    let var = tokens
        .next()
        .ok_or_else(|| ParseError::OutOfTokens((0, 0).into()))?;

    // get identifier
    let identifier;
    if let Some(next) = tokens.next() {
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
    if let Some(next) = tokens.next() {
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
    if let Some(next) = tokens.next() {
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
    if let Some(next) = tokens.next() {
        if matches!(next.tokentype(), Assign) {
            // get initializer expression
            let initializer = expression(tokens)?;
            let span = StartEndSpan::new(var.span.start, initializer.span.end);
            // get semicolon
            if let Some(_token) = tokens.next_if(|token| matches!(token.tokentype(), Semicolon)) {
                return Ok(Statement::new(Stmt::VariableDefinition(Variable::new(
                    &identifier,
                    kind,
                    Some(initializer),
                    span,
                ))));
            } else {
                return Err(ParseError::MissingSemicolon(span.into()));
            };
        } else if matches!(next.tokentype(), Semicolon) {
            return Ok(Statement::new(Stmt::VariableDefinition(Variable::new(
                &identifier,
                kind,
                None,
                StartEndSpan::new(var.span.start, next.span.end - 1),
            ))));
        } else if matches!(next.tokentype(), Equal) {
            // Help the user: if we find an Equal operator after the type initializer, the user probably meant to use Assign
            return Err(ParseError::ExpectedWalrus((next.span).into()));
        } else {
            return Err(ParseError::MissingSemicolon(
                StartEndSpan::new(var.span.start, next.span.end - 1).into(),
            ));
        };
    }
    Err(ParseError::OutOfTokens((0, 0).into()))
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
    assignment(tokens)
}

pub fn assignment(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, ParseError> {
    let mut expr = and(tokens)?;
    let spanstart = expr.span.start;
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        // Assignment
        if matches!(tokentype, Assign) {
            let assign = next.clone();
            tokens.next();
            let right = and(tokens)?;
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

pub fn and(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expression, ParseError> {
    let mut expr = equality(tokens)?;
    let spanstart = expr.span.start;
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, RawToken::And) {
            let operator = next.clone();
            tokens.next();
            let right = comparison(tokens)?;
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
            RawToken::Identifier(name) => {
                Ok(Expression::new(Expr::VariableUsage(name), token.span))
            }
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
            _ => Err(ParseError::ExpectedExpression(
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
