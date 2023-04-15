use std::fs;

use lexer::Lexer;
use syntax::Syntax;

mod lexer;
mod tokens;
mod syntax;
mod tests;

fn main() {
    let lexer = Lexer::new("hello+world");
    println!("{:?}", Syntax::new(lexer).check_validity());
}