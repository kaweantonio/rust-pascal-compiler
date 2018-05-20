use lexer;
use std::{thread, time};

// static mut indice: usize = 0;

// #[derive(Debug, Clone)]
// pub enum Terminais {
//     NUMB, // Numeros
//     STRING, // Cadeia de caracteres
//     IDEN, // Identificador
//     COIDEN, // Identificador de constantes
//     FIIDEN, // Identificador de fields
//     VAIDEN, // Identificador de variaveis
//     FUIDEN, // Identificador de funções
//     TYIDEN, // Identificador de tipos
//     PRIDEN, // Identificador de procedimentos
// }

// static mut tam: usize = lexer::tabelaToken.lock().unwrap().len();

pub fn Sintatico(){
    lexer::Lexico();

}
