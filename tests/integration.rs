#[cfg(test)]
mod valid {
    use minipl::lexing::scan;
    use minipl::parsing::Parser;
    use minipl::visitors::Interpreter;

    #[test]
    fn empty() {
        let source = include_str!("sources/valid/empty.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn newline() {
        let source = include_str!("sources/valid/newline.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn comment() {
        let source = include_str!("sources/valid/comment.minipl");
        let tokens = scan(source).unwrap();
        dbg!(&tokens);
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse();
        dbg!(&parsed);
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed.unwrap()).unwrap();
    }

    #[test]
    fn helloworld() {
        let source = include_str!("sources/valid/helloworld.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn hello_less_world() {
        let source = include_str!("sources/valid/hello_less_world.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn hello_plus_world() {
        let source = include_str!("sources/valid/hello_plus_world.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn one_plus_two_times_three() {
        let source = include_str!("sources/valid/one_plus_two_times_three.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn var_with_assign() {
        let source = include_str!("sources/valid/var_with_assign.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn var_without_assign() {
        let source = include_str!("sources/valid/var_without_assign.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn var_bool() {
        let source = include_str!("sources/valid/var_bool.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn var_string() {
        let source = include_str!("sources/valid/var_string.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn logical_and() {
        let source = include_str!("sources/valid/logical_and.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn for_zero_to_ten_print() {
        let source = include_str!("sources/valid/for_zero_to_ten_print.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn assert_true() {
        let source = include_str!("sources/valid/for_zero_to_ten_print.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn assert_truthy_comparison() {
        let source = include_str!("sources/valid/assert_truthy_comparison.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }
}

#[cfg(test)]
mod invalid {
    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use minipl::lexing::scan;
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
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();

        if let Err(report) = interpreter.eval(&parsed) {
            let report = fmt_report(report);
            dbg!(&report);
            assert!(report.contains("Variable error"));
            assert!(report.contains(
                "Try removing the latter `var` to reassign, or use a different identifier"
            ));
        } else {
            panic!()
        };
    }

    #[test]
    fn var_equal_not_walrus() {
        let source = include_str!("sources/invalid/var_equal_not_walrus.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(matches!(result, Err(ParseError::ExpectedWalrus(_))));
    }

    #[test]
    fn assign_to_nonvariable() {
        let source = include_str!("sources/invalid/assign_to_nonvariable.minipl");
        let tokens = scan(source).unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(matches!(result, Err(ParseError::AssignToNonVariable(_, _))));
    }
}
