# List the available recipes
default:
  @just --list --unsorted

doc:
  cargo doc --no-deps

# Run the CLI application with the given parameters
cli *ARGS:
  cargo run --bin minipl --release -- {{ARGS}}

# Print the abstract syntax tree for the program at the given path
ast PATH *ARGS:
  cargo run --bin minipl --release -- ast {{PATH}} {{ARGS}}

# Run the lexer for the program at the given path
lex PATH *ARGS:
  cargo run --bin minipl --release -- lex {{PATH}} {{ARGS}}

# Run the program at the given path
run PATH *ARGS:
  cargo run --bin minipl --release -- run {{PATH}} {{ARGS}}

# Print the abstract syntax tree for all the examples available in the repository
ast-all *ARGS:
  for example in $(ls examples/ |grep minipl); \
  do just cli ast examples/$example {{ARGS}}; \
  done;

# Run the lexer for all the examples available in the repository
lex-all *ARGS:
  for example in $(ls examples/ |grep minipl); \
  do just cli lex examples/$example {{ARGS}}; \
  done;

# Run all the examples available in the repository
run-all *ARGS:
  for example in $(ls examples/ |grep minipl); \
  do just cli run examples/$example {{ARGS}}; \
  done;
