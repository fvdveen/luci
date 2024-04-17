use crate::span::{Span, Symbol};

use super::token::{Delimiter, Token, TokenKind};

const EOF_CHAR: char = '\0';

pub struct Lexer<'a> {
    src: &'a str,
    ch: char,
    pos: usize,
    rd_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Lexer {
            src,
            ch: EOF_CHAR,
            pos: 0,
            rd_pos: 0,
        };

        lexer.bump();

        lexer
    }

    fn bump(&mut self) {
        if self.rd_pos < self.src.len() {
            let src = &self.src[self.rd_pos..];
            let mut chars = src.chars();
            self.pos = self.rd_pos;

            let ch = chars.next().unwrap();
            let w = ch.len_utf8();

            self.rd_pos += w;
            self.ch = ch;
        } else {
            self.pos = self.src.len();
            self.ch = EOF_CHAR;
        }
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while self.ch != EOF_CHAR && predicate(self.ch) {
            self.bump();
        }
    }

    pub fn next(&mut self) -> Token {
        use Delimiter::*;
        use TokenKind::*;
        self.skip_whitespace();

        let start = self.pos;
        let ch = self.ch;
        let token_kind = match ch {
            c if is_id_start(c) => {
                self.bump();
                self.ident(start)
            }
            _ => {
                self.bump();
                match ch {
                    EOF_CHAR => Eof,
                    '"' => self.string(start),
                    '(' => OpenDelim(Paren),
                    ')' => CloseDelim(Paren),
                    '{' => OpenDelim(Brace),
                    '}' => CloseDelim(Brace),
                    '[' => OpenDelim(Bracket),
                    ']' => CloseDelim(Bracket),
                    ';' => Semi,
                    _ => todo!("lex {}", ch),
                }
            }
        };

        Token::new(token_kind, Span::new(start, self.pos))
    }

    fn skip_whitespace(&mut self) {
        self.eat_while(is_whitespace);
    }

    fn ident(&mut self, start: usize) -> TokenKind {
        self.eat_while(is_id_continue);

        TokenKind::Symbol(Symbol::intern(&self.src[start..self.pos]))
    }

    fn string(&mut self, start: usize) -> TokenKind {
        self.eat_while(|ch| ch != '"');
        // Consume closing '"'
        self.bump();

        TokenKind::Symbol(Symbol::intern(&self.src[start..self.pos]))
    }
}

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

/// True if `c` is valid as a first character of an identifier.
pub fn is_id_start(c: char) -> bool {
    // This is XID_Start OR '_' (which formally is not a XID_Start).
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

/// True if `c` is valid as a non-first character of an identifier.
pub fn is_id_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

pub struct LexerIterator<'a> {
    lexer: Lexer<'a>,
    seen_eof: bool,
}

impl Iterator for LexerIterator<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.lexer.next();
        if tok.kind == TokenKind::Eof {
            if !self.seen_eof {
                self.seen_eof = true;
                Some(tok)
            } else {
                None
            }
        } else {
            Some(tok)
        }
    }
}

impl<'a> IntoIterator for Lexer<'a> {
    type Item = Token;
    type IntoIter = LexerIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LexerIterator {
            lexer: self,
            seen_eof: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parse::token::{Token, TokenKind},
        span::{Span, Symbol},
    };

    use super::Lexer;

    #[test]
    fn lex_symbol() {
        let src = "abcdefg";
        let sym = Symbol::intern("abcdefg");

        let mut lexer = Lexer::new(src);

        let expected_tokens = vec![
            Token::new(TokenKind::Symbol(sym), Span::new(0, 7)),
            Token::new(TokenKind::Eof, Span::new(7, 7)),
        ];

        for expected in expected_tokens {
            let actual = lexer.next();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn skip_whitespace() {
        let src = "\t\t\n\r    abcdefg";
        let sym = Symbol::intern("abcdefg");

        let mut lexer = Lexer::new(src);

        let expected_tokens = vec![
            Token::new(TokenKind::Symbol(sym), Span::new(8, 15)),
            Token::new(TokenKind::Eof, Span::new(15, 15)),
        ];

        for expected in expected_tokens {
            let actual = lexer.next();
            assert_eq!(expected, actual);
        }
    }
}
