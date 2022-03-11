use miette::SourceSpan;

/// The span i.e. the location descriptor of the token, in terms of bytes in the source code.
///
/// A tuple of `(start, end)`, start inclusive, end exclusive.
/// An example single-letter token `x` would have span `(0,1)`, a string token `"abc"` would have span `(0,5)`.
///
/// There is also a conversion method that converts from `(start, end)` to `(start, length)` style [`SourceSpan`] used in [`miette`].

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StartEndSpan {
    /// Start location of the span, as a byte offset, inclusive
    pub start: usize,
    /// End location of the span, as a byte offset, exclusive
    pub end: usize,
}

impl StartEndSpan {
    /// Creates a new StartEndSpan given the two byte offsets
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl From<StartEndSpan> for SourceSpan {
    fn from(val: StartEndSpan) -> Self {
        SourceSpan::new(val.start.into(), (val.end - val.start).into())
    }
}
