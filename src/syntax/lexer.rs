//! The lexer module implements a lexer for the monkey programming language, reading
//! the raw source text and emitting tokens.

use crate::syntax::token::{Kind, Token};
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'src> {
    /// Source text
    src: &'src str,

    /// Remaining characters
    chars: Chars<'src>,
}

impl<'src> Lexer<'src> {
    /// Construct a new lexer, scanning through `src`.
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            chars: src.chars(),
        }
    }

    /// lex returns the next Token from the input.
    pub fn lex(&mut self) -> Token {
        let start = self.offset();
        let kind = self.next_kind();
        let end = self.offset();

        Token { kind, start, end }
    }

    fn next_kind(&mut self) -> Kind {
        match self.chars.next() {
            Some('=') => Kind::Assign,
            Some(';') => Kind::Semicolon,
            Some('(') => Kind::OpenParen,
            Some(')') => Kind::CloseParen,
            Some(',') => Kind::Comma,
            Some('+') => Kind::Plus,
            Some('{') => Kind::OpenBrace,
            Some('}') => Kind::CloseBrace,
            None => Kind::Eof,
            _ => Kind::Error,
        }
    }

    fn offset(&self) -> usize {
        self.src.len() - self.chars.as_str().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basics() {
        let src = "=+(){},;";
        let mut lexer = Lexer::new(src);

        let mut got: Vec<Token> = vec![];

        loop {
            let tok = lexer.lex();
            got.push(tok);
            if tok.kind == Kind::Eof || tok.kind == Kind::Error {
                break;
            }
        }

        let want: Vec<Token> = vec![
            Token {
                kind: Kind::Assign,
                start: 0,
                end: 1,
            },
            Token {
                kind: Kind::Plus,
                start: 1,
                end: 2,
            },
            Token {
                kind: Kind::OpenParen,
                start: 2,
                end: 3,
            },
            Token {
                kind: Kind::CloseParen,
                start: 3,
                end: 4,
            },
            Token {
                kind: Kind::OpenBrace,
                start: 4,
                end: 5,
            },
            Token {
                kind: Kind::CloseBrace,
                start: 5,
                end: 6,
            },
            Token {
                kind: Kind::Comma,
                start: 6,
                end: 7,
            },
            Token {
                kind: Kind::Semicolon,
                start: 7,
                end: 8,
            },
            Token {
                kind: Kind::Eof,
                start: 8,
                end: 8,
            },
        ];

        assert_eq!(got, want);
    }
}
