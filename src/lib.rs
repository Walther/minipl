//! This library contains all the necessary functionality for interpreting the Mini-PL language as described on the Spring 2022 Compilers course at University of Helsinki.

// Lints
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::unwrap_used)]
#![deny(explicit_outlives_requirements)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]

// High-Level stuff

/// The lexing for the Mini-PL language
pub mod lexing;
/// The parsing for the Mini-PL language
pub mod parsing;
/// The tokens of the Mini-PL language
pub mod tokens;

// Plumbing

/// The visitors for the Mini-PL language
pub mod visitors;

/// The runtime features of Mini-PL language
pub mod runtime;

/// Internal span helper
mod span;
pub use span::StartEndSpan;
