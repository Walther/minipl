//! This library contains all the necessary functionality for interpreting the Mini-PL language as described on the Spring 2022 Compilers course at University of Helsinki.

// Lints
#![deny(clippy::all)]
#![deny(explicit_outlives_requirements)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

/// The lexer for the Mini-PL language
pub mod lexing;
/// The parsing for the Mini-PL language
pub mod parsing;
/// The raw tokens of the Mini-PL language
pub mod rawtoken;
/// The tokens of the Mini-PL language
pub mod token;
