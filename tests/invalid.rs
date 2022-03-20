use minipl::lexing::Lexer;
use minipl::parsing::{ParseError, Parser};
use minipl::runtime::RuntimeError;
use minipl::visitors::Interpreter;

#[test]
fn re_declaration() {
    let source = include_str!("sources/invalid/re_declaration.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(&parsed);
    assert!(matches!(
        result,
        Err(RuntimeError::VariableReDeclaration(_))
    ));
}

#[test]
fn var_equal_not_walrus() {
    let source = include_str!("sources/invalid/var_equal_not_walrus.minipl");
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
    let source = include_str!("sources/invalid/assign_to_nonvariable.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(matches!(result, Err(ParseError::AssignToNonVariable(_, _))));
}
