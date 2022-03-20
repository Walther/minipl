use minipl::lexing::Lexer;
use minipl::parsing::Parser;
use minipl::visitors::Interpreter;

#[test]
fn empty() {
    let source = include_str!("sources/valid/empty.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn newline() {
    let source = include_str!("sources/valid/newline.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn comment() {
    let source = include_str!("sources/valid/comment.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed.unwrap()).unwrap();
}

#[test]
fn multiline_comment_singleline() {
    let source = include_str!("sources/valid/multiline_comment_singleline.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed.unwrap()).unwrap();
}

#[test]
fn multiline_comment() {
    let source = include_str!("sources/valid/multiline_comment.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed.unwrap()).unwrap();
}

#[test]
fn multiline_comment_nested() {
    let source = include_str!("sources/valid/multiline_comment_nested.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed.unwrap()).unwrap();
}

#[test]
fn helloworld() {
    let source = include_str!("sources/valid/helloworld.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn hello_less_world() {
    let source = include_str!("sources/valid/hello_less_world.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn hello_plus_world() {
    let source = include_str!("sources/valid/hello_plus_world.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn one_plus_two_times_three() {
    let source = include_str!("sources/valid/one_plus_two_times_three.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn var_with_assign() {
    let source = include_str!("sources/valid/var_with_assign.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn var_without_assign() {
    let source = include_str!("sources/valid/var_without_assign.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn var_bool() {
    let source = include_str!("sources/valid/var_bool.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn var_string() {
    let source = include_str!("sources/valid/var_string.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn logical_and() {
    let source = include_str!("sources/valid/logical_and.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn for_zero_to_ten_print() {
    let source = include_str!("sources/valid/for_zero_to_ten_print.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn assert_true() {
    let source = include_str!("sources/valid/for_zero_to_ten_print.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}

#[test]
fn assert_truthy_comparison() {
    let source = include_str!("sources/valid/assert_truthy_comparison.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.eval(&parsed).unwrap();
}
