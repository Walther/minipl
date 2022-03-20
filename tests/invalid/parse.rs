use minipl::lexing::Lexer;
use minipl::parsing::{ParseError, Parser};

#[test]
fn var_equal_not_walrus() {
    let source = include_str!("../sources/invalid/var_equal_not_walrus.minipl");
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
fn assign_to_nonvariable() {
    let source = include_str!("../sources/invalid/assign_to_nonvariable.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::AssignToNonVariable(_, _))));
}
