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
    // TODO: integration tests for invalid code
}
