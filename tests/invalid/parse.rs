use minipl::lexing::Lexer;
use minipl::parsing::{ParseError, Parser};

/*
#[test]
fn template() {
    let source = include_str!("../sources/invalid/template.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::Template(_))));
}
*/

#[test]
fn missing_paren() {
    let source = include_str!("../sources/invalid/missing_paren.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::MissingParen(_))));
}

#[test]
fn expected_expression() {
    let source = include_str!("../sources/invalid/expected_expression.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::ExpectedExpression(_, _))));
}

#[test]
fn expected_identifier() {
    let source = include_str!("../sources/invalid/expected_identifier.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::ExpectedIdentifier(_, _))));
}

#[test]
fn expected_type_annotation() {
    let source = include_str!("../sources/invalid/expected_type_annotation.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(
        result,
        Err(ParseError::ExpectedTypeAnnotation(_, _))
    ));
}

#[test]
fn expected_assign() {
    let source = include_str!("../sources/invalid/expected_assign.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(
        result,
        Err(ParseError::ExpectedAssignFoundToken(_, _))
    ));
}

#[test]
fn assign_to_nonvariable() {
    let source = include_str!("../sources/invalid/assign_to_nonvariable.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::AssignToNonVariable(_, _))));
}

#[test]
fn read_to_nonvariable() {
    let source = include_str!("../sources/invalid/read_to_nonvariable.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::ReadToNonVariable(_, _))));
}

#[test]
fn expected_assign_found_equal() {
    let source = include_str!("../sources/invalid/expected_assign_found_equal.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(
        result,
        Err(ParseError::ExpectedAssignFoundEqual(_))
    ));
}

#[test]
fn missing_semicolon() {
    let source = include_str!("../sources/invalid/missing_semicolon.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::MissingSemicolon(_))));
}

#[test]
fn for_missing_variable() {
    let source = include_str!("../sources/invalid/for_missing_variable.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::ForMissingVariable(_, _))));
}

#[test]
fn for_missing_range() {
    let source = include_str!("../sources/invalid/for_missing_range.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    dbg!(&tokens);
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::ForMissingRange(_, _))));
}

#[test]
fn for_missing_in() {
    let source = include_str!("../sources/invalid/for_missing_in.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::ForMissingIn(_, _))));
}

#[test]
fn for_missing_do() {
    let source = include_str!("../sources/invalid/for_missing_do.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::ForMissingDo(_, _))));
}

#[test]
fn end_missing_for() {
    let source = include_str!("../sources/invalid/end_missing_for.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::EndMissingFor(_, _))));
}
