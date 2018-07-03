#[macro_use] extern crate lazy_static;
#[macro_use] extern crate text_io;
pub mod lexer;
mod parser;

fn main() {
    parser::ASD();
    //lexer::Lexico();
}
