# List the available recipes
default:
  @just --list --unsorted

# Run the CLI application with the given parameters
cli *ARGS:
  cargo run --bin minipl --release -- {{ARGS}}

# Run the program at the given path
run PATH *ARGS:
  cargo run --bin minipl --release -- run {{PATH}} {{ARGS}}

# Lex the program at the given path
lex PATH *ARGS:
  cargo run --bin minipl --release -- lex {{PATH}} {{ARGS}}

# Run all the examples available in the repository
run-all:
  for example in $(ls examples/ |grep minipl); \
  do just cli run examples/$example; \
  done;
