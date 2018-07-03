use lexer::Tokens;
use parser;
use std::sync::Mutex;

lazy_static! {
    pub static ref codigoMEPA: Mutex<Vec<String>> = Mutex::new(Vec::<String>::new());
}

pub fn MEPA(){
    parser::ASD();
}
