#[cfg(test)]
mod valid {
    use minipl::lexing::scan;
    use minipl::parsing::parse;
    use minipl::visitors::Interpreter;

    #[test]
    fn helloworld() {
        let source = include_str!("sources/valid/helloworld.minipl");
        let tokens = scan(source).unwrap();
        let parsed = parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn one_plus_two_times_three() {
        let source = include_str!("sources/valid/one_plus_two_times_three.minipl");
        let tokens = scan(source).unwrap();
        let parsed = parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn var_with_assign() {
        let source = include_str!("sources/valid/var_with_assign.minipl");
        let tokens = scan(source).unwrap();
        let parsed = parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn var_without_assign() {
        let source = include_str!("sources/valid/var_without_assign.minipl");
        let tokens = scan(source).unwrap();
        let parsed = parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn var_bool() {
        let source = include_str!("sources/valid/var_bool.minipl");
        let tokens = scan(source).unwrap();
        let parsed = parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }

    #[test]
    fn var_string() {
        let source = include_str!("sources/valid/var_string.minipl");
        let tokens = scan(source).unwrap();
        let parsed = parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval(&parsed).unwrap();
    }
}

#[cfg(test)]
mod invalid {
    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use minipl::lexing::scan;
    use minipl::parsing::{parse, ParseError};
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
        let parsed = parse(tokens).unwrap();
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
        let result = parse(tokens);
        assert!(matches!(result, Err(ParseError::ExpectedWalrus(_))));
    }
}
