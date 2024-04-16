use std::fs;

mod parse;
mod span;

fn main() {
    let src = fs::read_to_string("examples/main.luci").unwrap();

    parse::lex_file(&src);
}
