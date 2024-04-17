use self::lexer::Lexer;

mod lexer;
mod token;

pub fn lex_file(src: &str) {
    let lexer = Lexer::new(src);
    let tokens = lexer.into_iter().collect::<Vec<_>>();

    println!("{:#?}", tokens);
}
