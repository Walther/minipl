#![allow(missing_docs)] // TODO: document

mod expression;
use expression::*;
use std::iter::Peekable;

use crate::lexing::RawToken::{
    Bang, Equal, False, Less, Minus, Number, ParenLeft, ParenRight, Plus, Slash, Star, Text, True,
};
use crate::lexing::Token;

use anyhow::{anyhow, Error};

// Unstable syntax
// pub type Tokens = Peekable<impl Iterator<Item = Token>>;

pub fn parse(tokens: Vec<Token>) -> Result<Expr, Error> {
    let mut tokens = tokens.iter().cloned().peekable();
    expression(&mut tokens)
}

pub fn expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, Error> {
    equality(tokens)
}

pub fn equality(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, Error> {
    let mut expr = comparison(tokens)?;
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Equal) {
            let operator = next.clone();
            tokens.next();
            let right = comparison(tokens)?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        } else {
            break;
        }
    }
    Ok(expr)
}

pub fn comparison(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, Error> {
    let mut expr = term(tokens)?;
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Less) {
            let operator = next.clone();
            tokens.next();
            let right = term(tokens)?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        } else {
            break;
        }
    }
    Ok(expr)
}

pub fn term(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, Error> {
    let mut expr = factor(tokens)?;
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Minus | Plus) {
            let operator = next.clone();
            tokens.next();
            let right = factor(tokens)?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        } else {
            break;
        }
    }
    Ok(expr)
}

pub fn factor(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, Error> {
    let mut expr = unary(tokens)?;
    while let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Slash | Star) {
            let operator = next.clone();
            tokens.next();
            let right = unary(tokens)?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        } else {
            break;
        }
    }
    Ok(expr)
}

pub fn unary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, Error> {
    if let Some(next) = tokens.peek() {
        let tokentype = next.tokentype();
        if matches!(tokentype, Bang | Minus) {
            let operator = next.clone();
            tokens.next();
            let right = unary(tokens)?;
            return Ok(Expr::Unary(Unary::new(operator, right)));
        }
    } else {
        // What should happen here?
        todo!()
    }

    primary(tokens)
}

pub fn primary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, Error> {
    // At a terminal value, we need to always consume the token ?
    // TODO: verify
    if let Some(token) = tokens.next() {
        let tokentype = token.tokentype();
        match tokentype {
            False | True | Number(_) | Text(_) => Ok(Expr::Literal(Literal::new(token))),
            ParenLeft => {
                let expr = expression(tokens)?;
                if let Some(_token) = tokens.next_if(|token| token.tokentype() == ParenRight) {
                    Ok(Expr::Grouping(Grouping::new(expr)))
                } else {
                    Err(anyhow!("Expected ) after expression"))
                }
            }
            _ => Err(anyhow!("Expected expression, found token: {:?}", token)),
        }
    } else {
        return Err(anyhow!("Ran out of tokens?"));
    }
}

#[cfg(test)]
mod tests {
    use crate::parsing::*;

    #[test]
    fn literal_one() {
        let one = Token::new(Number(1), (0, 1));
        let parsed = parse(vec![one.clone()]).unwrap();
        let expected = Expr::Literal(Literal::new(one));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn one_equals_one() {
        let one1 = Token::new(Number(1), (0, 1));
        let equal = Token::new(Equal, (1, 2));
        let one2 = Token::new(Number(1), (2, 3));

        let tokens = vec![one1.clone(), equal.clone(), one2.clone()];

        let parsed = parse(tokens).unwrap();
        let expected = Expr::Binary(Binary::new(
            Expr::Literal(Literal::new(one1)),
            equal,
            Expr::Literal(Literal::new(one2)),
        ));
        assert_eq!(parsed, expected);
    }
}
