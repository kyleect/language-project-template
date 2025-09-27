//! The lexer and associated types

use logos::Logos;
use std::ops::Range;

use crate::{
    errors::{ExprErrorS, LexicalError},
    span::Spanned,
};

/// Parse source code in to a list of [`Token`].
pub fn lex(source: &str) -> Vec<Result<(usize, Token, usize), ExprErrorS>> {
    let lexer: Lexer<'_> = Lexer::new(&source);
    let tokens: Vec<Result<(usize, Token, usize), ExprErrorS>> = lexer.collect::<Vec<_>>();

    tokens
}

/// Converts a [`String`] source in to a vector of [`Token`]
#[derive(Debug)]
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
    pending: Option<(usize, Token, usize)>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            inner: Token::lexer(source),
            pending: None,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<(usize, Token, usize), ExprErrorS>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.pending.take() {
            return Some(Ok(token));
        }

        let token = self.inner.next()?;

        {
            let Range { start, end } = self.inner.span();

            Some(
                token
                    .map(|token| (start, token, end))
                    .map_err(|(err, err_span)| (err.into(), err_span)),
            )
        }
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = Spanned<LexicalError>)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("(")]
    LParan,

    #[token(")")]
    RParan,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("/")]
    Slash,

    #[token("*")]
    Astrisk,

    #[regex(r#"[0-9]+(\.[0-9]+)?"#, lex_number)]
    Number(f64),
}

fn lex_number(lexer: &mut logos::Lexer<Token>) -> f64 {
    let slice = lexer.slice();
    slice.parse::<f64>().expect("should be a parsable number")
}
