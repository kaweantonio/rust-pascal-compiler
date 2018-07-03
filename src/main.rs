#[macro_use] extern crate lazy_static;
#[macro_use] extern crate text_io;
pub mod lexer;
mod parser;
mod geradorMEPA;

fn main() {
    geradorMEPA::MEPA();
    //parser::ASD();
    //lexer::Lexico();
}
