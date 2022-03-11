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
        interpreter.eval_all(&parsed).unwrap();
    }

    #[test]
    fn one_plus_two_times_three() {
        let source = include_str!("sources/valid/one_plus_two_times_three.minipl");
        let tokens = scan(source).unwrap();
        let parsed = parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.eval_all(&parsed).unwrap();
    }
}

#[cfg(test)]
mod invalid {
    // TODO: integration tests for invalid code
}
