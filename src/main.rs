use crate::span::Symbol;

mod span;

fn main() {
    let greet = Symbol::intern("Hello, world!");
    println!("{}", greet.as_str());
}
