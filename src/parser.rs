use lexer;

#[derive(Debug, Clone)]
pub enum Terminais {
    NUMB, // Numeros
    STRING, // Cadeia de caracteres
    IDEN, // Identificador
    COIDEN, // Identificador de constantes
    FIIDEN, // Identificador de fields
    VAIDEN, // Identificador de variaveis
    FUIDEN, // Identificador de funções
    TYIDEN, // Identificador de tipos
    PRIDEN, // Identificador de procedimentos
}

pub fn Sintatico(){
    let tabelaToken = lexer::Lexico();
    let val = tabelaToken.len();

    for i in 0..val {

        let ref temp = tabelaToken[i];

        println!("{:?}",*temp);

    }
}
