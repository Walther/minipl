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
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
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
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
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
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
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
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
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
            return Ok(Expr::Unary(Unary::new(operator, Box::new(right))));
        }
    } else {
        // What should happen here?
        todo!()
    }

    primary(tokens)
}

pub fn primary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr, Error> {
    if let Some(token) = tokens.peek() {
        let tokentype = token.tokentype();
        match tokentype {
            False | True | Number(_) | Text(_) => Ok(Expr::Literal(Literal::new(token.clone()))),
            ParenLeft => {
                let expr = expression(tokens)?;
                if let Some(_token) = tokens.next_if(|token| token.tokentype() == ParenRight) {
                    Ok(Expr::Grouping(Grouping::new(Box::new(expr))))
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
        let one = Number(1);
        let location = (0, 1);
        let one = Token::new(one, location);
        let parsed = parse(vec![one.clone()]).unwrap();
        let expected = Expr::Literal(Literal::new(one));
        dbg!(&parsed);
        assert_eq!(parsed, expected);
    }

    /*
    // Not implemented yet
    #[test]
    fn one_equals_one() {
        let one1 = Number(1);
        let location = (0, 1);
        let one1 = Token::new(one1, location);

        let equal = Equal;
        let location = (1, 2);
        let equal = Token::new(equal, location);

        let one2 = Number(1);
        let location = (2, 3);
        let one2 = Token::new(one2, location);

        let tokens = vec![one1.clone(), equal.clone(), one2.clone()];

        let parsed = parse(tokens).unwrap();
        let expected = Expr::Binary(Binary::new(
            Box::new(Expr::Literal(Literal::new(one1))),
            equal,
            Box::new(Expr::Literal(Literal::new(one2))),
        ));
        dbg!(&parsed);
        assert_eq!(parsed, expected);
    }
    */
}
