use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Clone)]
pub enum Tokens {
    // keywords
    Programa,
    Procedure,
    Function,
    Div,
    Or,
    And,
    Not,
    If,
    Then,
    Else,
    Of,
    While,
    Do,
    Begin,
    End,
    Read,
    Write,
    Var,
    Array,

    // Tipos
    True,
    False,
    Char,
    CadeiCaracteres,
    Identificador,
    Inteiro,
    Float,

    // Simbolos
    Mais, // +
    Menos, // -
    Asterisco, // *
    BarraDir, // /
    Igual, // =
    Diferente, // <>
    Menor, // <
    Maior, // >
    MenorIgual, // <=
    MaiorIgual, // >=
    AbreParenteses, // (
    FechaParenteses, // )
    AbreColchete, // [
    FechaColchete, // ]
    Atribuicao, // :=
    Ponto, // .
    Virgula, // ,
    PontoVirgula, // ;
    DoisPontos, // :
    PontoPonto, // ..
    Apostrofo, // '
    Aspas, // "


    EspacoEmBrano,
    Commentario,
}

fn TabelaSimbolos<'a>() -> Vec<Vec::<String>> {
    let mut file = File::open("src/input.txt").expect("Não foi possível abrir o arquivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Não foi possível ler o arquivo");

    let mut aux = Vec::new();
    aux.push(Vec::<String>::new());
    let mut i = 0;
    let mut j = 0;

    for (indice, caractere) in contents.match_indices(|c: char| !(c.is_alphanumeric())) {
        if i != indice{
            aux[j].push((&contents[i..indice]).to_string());
        }

        if caractere == "\r" || caractere == "\n" {
            aux.push(Vec::<String>::new());
            j = j + 1;
        } else if caractere != "\t" && caractere != " "{
            aux[j].push(caractere.to_string());
        }

        i = indice + caractere.len();
    }

    if i < contents.len(){
        aux[j].push((&contents[i..]).to_string());
    }

    aux.retain(|x| x.len() > 0);

    return aux;
}

pub fn Lexico() {
    let mut result = TabelaSimbolos();

    println!("{:?}",result);
    println!("\n\n");
}
