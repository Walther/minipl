set shell := ["bash", "-uc"]

# List the available just recipes
default:
  @just --list --unsorted

# Build and install the minipl command line tool
install:
  cargo install --path .

# Show the help for the minipl command line tool
help:
  cargo run --bin minipl --release -- help

# Generate and open the documentation for the codebase
doc:
  cargo doc --no-deps --open

# Run all the tests of the codebase
test:
  cargo test

# Run the CLI application with the given parameters
cli *ARGS:
  cargo run --bin minipl --release -- {{ARGS}}

# Run the lexer for the program at the given path
lex PATH *ARGS:
  cargo run --bin minipl --release -- lex {{PATH}} {{ARGS}}

# Print the abstract syntax tree for the program at the given path
ast PATH *ARGS:
  cargo run --bin minipl --release -- ast {{PATH}} {{ARGS}}

# Run the program at the given path
run PATH *ARGS:
  cargo run --bin minipl --release -- run {{PATH}} {{ARGS}}

# Run the lexer for all the examples available in the repository
lex-all *ARGS:
  pushd tests/sources/valid; \
  for example in $(ls |grep minipl); \
    do just cli lex $(pwd)/$example {{ARGS}}; \
  done; \
  popd;

# Print the abstract syntax tree for all the examples available in the repository
ast-all *ARGS:
  pushd tests/sources/valid; \
  for example in $(ls |grep minipl); \
    do just cli ast $(pwd)/$example {{ARGS}}; \
  done; \
  popd;

# Run all the examples available in the repository
run-all *ARGS:
  pushd tests/sources/valid; \
  for example in $(ls |grep minipl); \
    do just cli run $(pwd)/$example {{ARGS}}; \
  done; \
  popd;
