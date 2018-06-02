#[macro_use]
extern crate lazy_static;
pub mod lexer;
mod parser;

fn main() {
    parser::ASD();
    //lexer::Lexico();
}
