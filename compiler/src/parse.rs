use self::{lexer::Lexer, token::TokenKind};

mod lexer;
mod token;

pub fn lex_file(src: &str) {
    let mut lexer = Lexer::new(src);

    let mut tokens = vec![];

    'lex: loop {
        let tok = lexer.next();
        tokens.push(tok);
        if tokens.last().unwrap().kind == TokenKind::Eof {
            break 'lex;
        }
    }

    println!("{:#?}", tokens)
}
