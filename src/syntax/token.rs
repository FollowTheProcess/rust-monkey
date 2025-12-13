//! The token module provides the set of lexical tokens available in monkey.

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Error represents an invalid token.
    Error,

    /// Eof signifies we've reached the end of the input.
    Eof,

    /// Assignment '='
    Assign,

    /// '+'
    Plus,

    /// ','
    Comma,

    /// ';'
    Semicolon,

    /// '('
    OpenParen,

    /// ')'
    CloseParen,

    /// '{'
    OpenBrace,

    /// '}'
    CloseBrace,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Error => "Error",
            Self::Eof => "EOF",
            Self::Assign => "Assign",
            Self::Plus => "Plus",
            Self::Comma => "Comma",
            Self::Semicolon => "Semicolon",
            Self::OpenParen => "OpenParen",
            Self::CloseParen => "CloseParen",
            Self::OpenBrace => "OpenBrace",
            Self::CloseBrace => "CloseBrace",
        };

        write!(f, "{text}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// The kind of token this is.
    pub kind: Kind,

    /// The start position of this token in src.
    pub start: usize,

    /// The end position of this token in src.
    pub end: usize,
}

impl Token {
    /// Is the token an EOF.
    pub fn is_eof(&self) -> bool {
        self.kind == Kind::Eof
    }

    /// Is the token an Error.
    pub fn is_error(&self) -> bool {
        self.kind == Kind::Error
    }
}

// TODO(@FollowTheProcess): Should this actually just show the literal? e.g. ',' for comma?
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Token::{} start={}, end={}>",
            self.kind, self.start, self.end
        )
    }
}
