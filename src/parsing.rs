#![allow(missing_docs)] // TODO: document

use std::iter::Peekable;

pub mod expression;
pub use expression::*;
pub mod statement;
pub use statement::*;
pub mod variable;
pub use variable::*;
pub mod forloop;
pub use forloop::*;

use crate::span::StartEndSpan;
use crate::tokens::RawToken::{
    self, Assert, Assign, Bang, Bool, Colon, End, Equal, False, For, Int, Less, Minus, Number,
    ParenLeft, ParenRight, Plus, Print, Range, Read, Semicolon, Slash, Star, Text, True, Var,
};
use crate::tokens::Token;
mod parse_error;
pub use parse_error::ParseError;

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

pub fn statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    if let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, For) {
            for_statement(tokens)
        } else if matches!(tokentype, Assert) {
            // consume the assert token
            tokens.next();
            assert_statement(tokens)
        } else if matches!(tokentype, Print) {
            // consume the print token
            tokens.next();
            print_statement(tokens)
        } else if matches!(tokentype, Read) {
            // consume the read token
            tokens.next();
            read_statement(tokens)
        } else {
            epxr_statement(tokens)
        }
    } else {
        Err(ParseError::NothingToParse((0, 0).into()))
    }
}

pub fn assert_statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    let expr = expression(tokens)?;
    if let Some(_token) = tokens.next_if(|token| matches!(token.tokentype(), Semicolon)) {
        Ok(Statement::new(Stmt::Assert(expr.clone()), expr.span))
    } else {
        Err(ParseError::MissingSemicolon(expr.span.into()))
    }
}

pub fn for_statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    // consume the for token // TODO: remove unwrap
    let start = tokens.next().unwrap();
    // variable name literal
    let name;
    if let Some(next) = tokens.next() {
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
    if let Some(next) = tokens.next() {
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
    let left = expression(tokens)?;
    // range literal
    if let Some(next) = tokens.next() {
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
    let right = expression(tokens)?;
    // do keyword
    if let Some(next) = tokens.next() {
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
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        // Have we found the end?
        if matches!(tokentype, End) {
            // consume the end token
            tokens.next();
            // expect to find for token
            if let Some(next) = tokens.next() {
                let tokentype = next.tokentype();
                match tokentype {
                    For => {
                        // expect to find semicolon
                        if let Some(_token) =
                            tokens.next_if(|token| matches!(token.tokentype(), Semicolon))
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
        let statement = statement(tokens)?;
        body.push(statement);
    }

    let last = body.last().unwrap(); // TODO: remove unwrap
    let span = StartEndSpan::new(start.span.start, last.span.end);
    Ok(Statement::new(
        Stmt::Forloop(Forloop::new(&name, left, right, body, span)),
        span,
    ))
}

pub fn print_statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    let expr = expression(tokens)?;
    if let Some(_token) = tokens.next_if(|token| matches!(token.tokentype(), Semicolon)) {
        Ok(Statement::new(Stmt::Print(expr.clone()), expr.span))
    } else {
        Err(ParseError::MissingSemicolon(expr.span.into()))
    }
}

pub fn read_statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    if let Some(token) = tokens.next() {
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
        if let Some(_token) = tokens.next_if(|token| matches!(token.tokentype(), Semicolon)) {
            Ok(Statement::new(Stmt::Read(name), token.span))
        } else {
            Err(ParseError::MissingSemicolon(expr.span.into()))
        }
    } else {
        Err(ParseError::OutOfTokens((0, 0).into()))
    }
}

pub fn epxr_statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, ParseError> {
    let expr = expression(tokens)?;
    if let Some(_token) = tokens.next_if(|token| matches!(token.tokentype(), Semicolon)) {
        Ok(Statement::new(Stmt::Expression(expr.clone()), expr.span))
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

        let parsed = parse(tokens).unwrap();
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
