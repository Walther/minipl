use minipl::lexing::Lexer;
use minipl::parsing::Parser;
use minipl::runtime::RuntimeError::*;
use minipl::visitors::Interpreter;

#[test]
fn re_declaration() {
    let source = include_str!("../sources/invalid/re_declaration.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(VariableReDeclaration(_))));
}
