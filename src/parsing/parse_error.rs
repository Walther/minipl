use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Parse error")]
#[diagnostic()]
pub enum ParseError {
    NothingToParse(
        #[label = "Nothing to parse. Source contained ignorable tokens only."] SourceSpan,
    ),
    MissingParen(#[label = "Expected ( after this grouping"] SourceSpan),
    ExpectedExpression(
        String,
        #[label = "Expected expression, found token {0}"] SourceSpan,
    ),
    ExpectedIdentifier(
        String,
        #[label = "Expected identifier, found token {0}"] SourceSpan,
    ),
    ExpectedTypeAnnotation(
        String,
        #[label = "Expected identifier, found token {0}"] SourceSpan,
    ),
    ExpectedAssignment(
        String,
        #[label = "Expected assignment operator :=, found token {0}"] SourceSpan,
    ),
    #[diagnostic(help("Usage: variable_name := new_value"))]
    AssignToNonVariable(
        String,
        #[label = "Expected assignment to variable, found token {0}"] SourceSpan,
    ),
    #[diagnostic(help("Usage: read variable_name"))]
    ReadToNonVariable(
        String,
        #[label = "Expected read to variable, found token {0}"] SourceSpan,
    ),
    #[diagnostic(help("Use the assignment operator := instead of = for declaring a variable"))]
    ExpectedWalrus(#[label = "Expected assignment operator `:=`, found `=`"] SourceSpan),
    OutOfTokens(#[label = "Ran out of tokens while parsing"] SourceSpan),
    MissingSemicolon(#[label = "Expected ; after statement"] SourceSpan),
    #[diagnostic(help("Usage: for x in a..b do \\n [body] \\n end for;"))]
    ForMissingVariable(
        String,
        #[label = "Expected variable name, found token {0}"] SourceSpan,
    ),
    #[diagnostic(help("Usage: for x in a..b do \\n [body] \\n end for;"))]
    ForMissingRange(
        String,
        #[label = "Expected range syntax `..`, found token {0}"] SourceSpan,
    ),
    #[diagnostic(help("Usage: for x in a..b do \\n [body] \\n end for;"))]
    ForMissingIn(
        String,
        #[label = "Expected keyword `in`, found token {0}"] SourceSpan,
    ),
    #[diagnostic(help("Usage: for x in a..b do \\n [body] \\n end for;"))]
    ForMissingDo(
        String,
        #[label = "Expected keyword `do`, found token {0}"] SourceSpan,
    ),
    #[diagnostic(help("Usage: for x in a..b do \\n [body] \\n end for;"))]
    EndMissingFor(
        String,
        #[label = "Expected keyword `for`, found token {0}"] SourceSpan,
    ),
}
