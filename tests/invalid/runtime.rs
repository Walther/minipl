use minipl::lexing::Lexer;
use minipl::parsing::Parser;
use minipl::runtime::RuntimeError::*;
use minipl::visitors::Interpreter;

/*
#[test]
fn template() {
    let source = include_str!("../sources/invalid/template.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(Template(_))));
}
*/

#[test]
fn as_numeric_failed() {
    let source = include_str!("../sources/invalid/as_numeric_failed.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(AsNumericFailed(_))));
}

#[test]
fn as_boolean_failed() {
    let source = include_str!("../sources/invalid/as_boolean_failed.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(AsBooleanFailed(_))));
}

#[test]
fn equal_type_mismatch() {
    let source = include_str!("../sources/invalid/equal_type_mismatch.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(EqualTypeMismatch(_, _, _, _))));
}

#[test]
fn less_type_mismatch() {
    let source = include_str!("../sources/invalid/less_type_mismatch.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(LessTypeMismatch(_, _, _, _))));
}

#[test]
fn assert_expr_not_truthy() {
    let source = include_str!("../sources/invalid/assert_expr_not_truthy.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(AssertExprNotTruthy(_))));
}

#[test]
fn assertion_failed() {
    let source = include_str!("../sources/invalid/assertion_failed.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(AssertionFailed(_))));
}

#[test]
fn for_end_larger() {
    let source = include_str!("../sources/invalid/for_end_larger.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(ForEndLarger(_, _))));
}

#[test]
fn for_end_nonnumeric() {
    let source = include_str!("../sources/invalid/for_end_nonnumeric.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(ForEndNonNumeric(_, _))));
}

#[test]
fn for_start_nonnumeric() {
    let source = include_str!("../sources/invalid/for_start_nonnumeric.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(ForStartNonNumeric(_, _))));
}

#[test]
fn plus_type_mismatch() {
    let source = include_str!("../sources/invalid/plus_type_mismatch.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(PlusTypeMismatch(_, _, _, _))));
}

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

#[test]
fn variable_assign_to_undeclared() {
    let source = include_str!("../sources/invalid/variable_assign_to_undeclared.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(VariableAssignToUndeclared(_))));
}

#[test]
fn variable_get_failed() {
    let source = include_str!("../sources/invalid/variable_get_failed.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(result, Err(VariableGetFailed(_))));
}

#[test]
fn variable_assign_type_mismatch() {
    let source = include_str!("../sources/invalid/variable_assign_type_mismatch.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(
        result,
        Err(VariableAssignTypeMismatch(_, _, _, _))
    ));
}

#[test]
fn variable_assign_type_mismatch2() {
    let source = include_str!("../sources/invalid/variable_assign_type_mismatch2.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.eval(&parsed);
    assert!(matches!(
        result,
        Err(VariableAssignTypeMismatch(_, _, _, _))
    ));
}
