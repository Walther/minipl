/// All raw tokens of the Mini-PL programming language.
#[derive(Debug, Clone, PartialEq)]
pub enum RawToken {
    // Single-character tokens
    /// `&`
    And,
    /// `!`
    Bang,
    /// `:`
    Colon,
    /// `=`
    Equal,
    /// `<`
    Less,
    /// `-`
    Minus,
    /// `(`
    ParenLeft,
    /// `)`
    ParenRight,
    /// `+`
    Plus,
    /// `;`
    Semicolon,
    /// `/`
    Slash,
    /// `*`
    Star,

    // Multi-character tokens
    /// `:=`
    Assign,
    /// `..`
    Range,

    // Literals
    /// Identifier, e.g. a variable name
    Identifier(String),
    /// Literal number. Internal type i64
    Number(i64),
    /// Text, i.e. a string
    Text(String),

    // Keywords
    /// `assert`
    Assert,
    /// `bool`
    Bool,
    /// `do`
    Do,
    /// `end`
    End,
    /// `false`
    False,
    /// `for`
    For,
    /// `in`
    In,
    /// `int`
    Int,
    /// `print
    Print,
    /// `read`
    Read,
    /// `string`
    String,
    /// `true`
    True,
    /// `var`
    Var,

    // Ignorables
    /// Comment type. Exists for the internal convenience of the lexer.
    Comment,
    /// Error type. Exists for propagating parser errors with helpful messages to the user.
    Error(String),
    /// Whitespace type. Exists for the internal convenience of the lexer.
    Whitespace,

    // End of file marker
    /// End of file marker. Exists for the internal convenience of the lexer.
    EOF,
}
