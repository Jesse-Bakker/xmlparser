use core::fmt;
use core::ops::{Deref, Range};


/// A string slice.
///
/// Like `&str`, but also contains the position in the input XML,
/// from which it was parsed.
#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrSpan<'a> {
    text: &'a str,
    start: usize,
}

impl<'a> From<&'a str> for StrSpan<'a> {
    #[inline]
    fn from(text: &'a str) -> Self {
        StrSpan {
            text,
            start: 0,
        }
    }
}

impl<'a> StrSpan<'a> {
    /// Constructs a new `StrSpan` from substring.
    #[inline]
    pub(crate) fn from_substr(text: &str, start: usize, end: usize) -> StrSpan {
        debug_assert!(start <= end);
        StrSpan { text: &text[start..end], start }
    }

    /// Returns a start position of the span.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns a end position of the span.
    #[inline]
    pub fn end(&self) -> usize {
        self.start + self.text.len()
    }

    /// Returns a end position of the span.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.start..self.end()
    }

    /// Returns a span slice.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        &self.text
    }

    /// Returns an underling string region as `StrSpan`.
    #[inline]
    pub(crate) fn slice_region(&self, start: usize, end: usize) -> StrSpan<'a> {
        StrSpan::from_substr(self.text, start, end)
    }
}

impl<'a> fmt::Debug for StrSpan<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StrSpan({:?} {}..{})", self.as_str(), self.start(), self.end())
    }
}

impl<'a> fmt::Display for StrSpan<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> Deref for StrSpan<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text
    }
}
