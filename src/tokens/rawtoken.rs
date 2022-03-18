/// All raw tokens (also known as lexemes) of the Mini-PL programming language.
#[derive(Debug, Clone, PartialEq)]
pub enum RawToken {
    // Single-character tokens
    /// `&` Logical AND operator
    And,
    /// `!` Logical NOT operator
    Bang,
    /// `:` Colon operator used for type ascription
    Colon,
    /// `=` Equal operator used for equality checking. Note: not used for assignment
    Equal,
    /// `<` Less operator used for comparison. Note: this is the only comparison operator
    Less,
    /// `-` Minus operator used for substraction
    Minus,
    /// `(` Left paren used for grouping
    ParenLeft,
    /// `)` Right paren used for grouping
    ParenRight,
    /// `+` Plus operator for addition
    Plus,
    /// `;` Semicolon used for terminating statements
    Semicolon,
    /// `/` Slash operator used for division
    Slash,
    /// `*` Star operator used for multiplication
    Star,

    // Multi-character tokens
    /// `:=` Assign operator used for assigning values to variables
    Assign,
    /// `..` Range operator used for defining ranges for for loops
    Range,

    // Literals
    /// Identifier, a name for a variable. Internally represented as a [String]
    Identifier(String),
    /// Literal number. Internally represented as an [i64]
    Number(i64),
    /// Literal string. Internally represented as a [String]
    Text(String),

    // Keywords
    /// `assert` used for evaluating truthy statements and stopping execution on false
    Assert,
    /// `bool` type keyword for boolean data
    Bool,
    /// `do` used in the for loop definitions as a keyword before the loop body begins
    Do,
    /// `end` used in the for loop definitions as an `end for` keyword pair
    End,
    /// `false` boolean literal
    False,
    /// `for` keyword for for loops
    For,
    /// `in` used in the for loop definitions as a keyword before the range definition
    In,
    /// `int` type keyword for numeric data
    Int,
    /// `print` keyword for printing to standard output
    Print,
    /// `read` keyword for reading a variable from standard input
    Read,
    /// `string` type keyword for string data
    String,
    /// `true` boolean literal
    True,
    /// `var` keyword for declaring a variable identifier
    Var,

    // Ignorables
    /// Comment type. Exists for the internal convenience of the lexer.
    Comment,
    /// Error type. Exists for propagating lexing errors with helpful messages to the user.
    Error(String),
    /// Whitespace type. Exists for the internal convenience of the lexer.
    Whitespace,

    // End of file marker
    /// End of file marker. Exists for the internal convenience of the lexer.
    EOF,
}
