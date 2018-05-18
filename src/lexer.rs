use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Clone)]
pub enum Tokens {
    // palavras reservadas
    And,
    Array,
    Asm,
    Begin,
    Case,
    Const,
    Div,
    Do,
    Else,
    End,
    For,
    Function,
    If,
    Not,
    Of,
    Or,
    Procedure,
    Program,
    Read,
    Reservada_String,
    Then,
    To,
    Type,
    Until,
    Var,
    While,
    With,
    Write,
    
    // Tipos de variável
    True,
    False,
    Char,
    Tipo_String,
    Identificador,
    Inteiro,
    Float,

    // Símbolos de pontuação
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


    // Simbolos de Relação
    Igual, // =
    Diferente, // <>
    Menor, // <
    Maior, // >
    MenorIgual, // <=
    MaiorIgual, // >=

    // Símbolos Aritméticos
    Mais, // +
    Menos, // -
    Multiplicacao, // *
    Divisao, // /
    Modulo, // %

    Comentario,
}

#[derive(Debug)]
pub struct Token {
    pub tok: String,
    pub tipo: Tokens,
    pub lin: i32,
    pub col: i32,
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

fn AnalisaLexico(tabela:&mut Vec<String>, linha: i32) -> Vec<Token> {
    let mut classificado = false;
    let num_tokens = tabela.len(); // número de tokens no Vector 

    unsafe {
        let mut prox_token = Token {
            tok: ("").to_string(),
            tipo: Tokens::Comentario,
            lin: 0,
            col: 0
        };

        let mut aux = Vec::<Token>::new();

        for token in tabela{
            prox_token = Token {
                tok: ("").to_string(),
                tipo: Tokens::Comentario,
                lin: 0,
                col: 0
            };
            // verifica se é reservada
            match token.to_lowercase().as_ref() {
                "and" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::And, lin: linha, col: 0};
                    classificado = true
                },
                "array" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Array, lin: linha, col: 0};
                    classificado = true
                },
                "asm" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Asm, lin: linha, col: 0};
                    classificado = true
                },
                "begin" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Begin, lin: linha, col: 0};
                    classificado = true
                },
                "case" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Case, lin: linha, col: 0};
                    classificado = true
                },
                "const" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Const, lin: linha, col: 0};
                    classificado = true
                },
                "div" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Div, lin: linha, col: 0};
                    classificado = true
                },
                "do" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Do, lin: linha, col: 0};
                    classificado = true
                },
                "else" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Else, lin: linha, col: 0};
                    classificado = true
                },
                "end" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::End, lin: linha, col: 0};
                    classificado = true
                },
                "for" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::For, lin: linha, col: 0};
                    classificado = true
                },
                "function" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Function, lin: linha, col: 0};
                    classificado = true
                },
                "if" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::If, lin: linha, col: 0};
                    classificado = true
                },
                "not" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Not, lin: linha, col: 0};
                    classificado = true
                },
                "of" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Of, lin: linha, col: 0};
                    classificado = true
                },
                "or" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Or, lin: linha, col: 0};
                    classificado = true
                },
                "procedure" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Procedure, lin: linha, col: 0};
                    classificado = true
                },
                "program" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Program, lin: linha, col: 0};
                    classificado = true
                },
                "read" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Read, lin: linha, col: 0};
                    classificado = true
                },
                "string" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Reservada_String, lin: linha, col: 0};
                    classificado = true
                },
                "then" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Then, lin: linha, col: 0};
                    classificado = true
                },
                "to" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::To, lin: linha, col: 0};
                    classificado = true
                },
                "type" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Type, lin: linha, col: 0};
                    classificado = true
                },
                "until" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Until, lin: linha, col: 0};
                    classificado = true
                },
                "var" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Var, lin: linha, col: 0};
                    classificado = true
                },
                "while" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::While, lin: linha, col: 0};
                    classificado = true
                },
                "with" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::With, lin: linha, col: 0};
                    classificado = true
                },
                "write" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Write, lin: linha, col: 0};
                    classificado = true
                },
                _ => classificado = false,
            }

            if (classificado == false){
                prox_token = Token{tok: token.to_string(), tipo: Tokens::Identificador, lin: linha, col: 0};
            }

            aux.push(prox_token);
        }

        return aux;
    }
}

pub fn Lexico() {
    let mut result = TabelaSimbolos();

    let mut tabelaToken = vec![Vec::<Token>::new()];

    println!("{:?}",result);
    println!("\n\n");

    for i in 0..result.len(){
        let aux = AnalisaLexico(&mut result[i], ((i+1) as i32));

        tabelaToken.push(aux)
    }

    println!("\n\n");

    for i in 1..tabelaToken.len(){
        println!("Linha: {0}", i);
        for j in 0..tabelaToken[i].len(){
            println!("{:?}", tabelaToken[i][j]);
        }
    }
}
