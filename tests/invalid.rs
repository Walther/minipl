use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use minipl::lexing::Lexer;
use minipl::parsing::{ParseError, Parser};
use minipl::visitors::Interpreter;

// TODO: even better UI testing
fn fmt_report(diag: Report) -> String {
    let mut out = String::new();

    GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor())
        .with_width(512)
        .render_report(&mut out, diag.as_ref())
        .unwrap();

    out
}

#[test]
fn re_declaration() {
    let source = include_str!("sources/invalid/re_declaration.minipl");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();

    if let Err(report) = interpreter.eval(&parsed) {
        let report = fmt_report(report);
        assert!(report.contains("Variable error"));
        assert!(report
            .contains("Try removing the latter `var` to reassign, or use a different identifier"));
    } else {
        panic!()
    };
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
