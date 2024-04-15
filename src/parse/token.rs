use crate::span::{Span, Symbol};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    Symbol(Symbol),

    OpenDelim(Delimiter),
    CloseDelim(Delimiter),
    Semi,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Delimiter {
    Paren,
    Brace,
    Bracket,
}
