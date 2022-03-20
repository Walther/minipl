# minipl

An interpreter for the Mini-PL programming language.

Course project for the Spring 2022 Compilers class at University of Helsinki.

## Requirements

The Minimum Supported Rust Version (MSRV) is `1.58.1`.

Make sure you have a sufficient Rust version installed using [rustup](https://rustup.rs/).

This project uses colorized unicode output. Make sure to use a terminal emulator / shell that supports colors and unicode.

## Usage

This project has a [Justfile](https://github.com/casey/just). If you have `just` installed, you can run `just` to list the available commands in this project, and run various forms of `just command --option args` to execute them.

If you do not have `just` installed, open the [Justfile](./Justfile) in your favorite text editor to see the various available commands that you can then copy to your shell manually. Note that e.g. the various `all` commands can be very handy for running against all of the available valid code samples.

Running `just install` aka `cargo install --path .` will build the project and install the `minipl` executable into your path. After that, running `minipl --help` will print you the latest command line tool help.
