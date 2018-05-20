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
    Boolean,
    Case,
    Char,
    Const,
    Div,
    Do,
    Else,
    End,
    False,
    For,
    Function,
    If,
    Integer,
    Not,
    Of,
    Or,
    Procedure,
    Program,
    Read,
    Reservada_String,
    Then,
    To,
    True,
    Type,
    Until,
    Var,
    While,
    With,
    Write,

    // Tipos de variável
    Identificador,
    Numero,

    // Símbolos de Pontuação
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

    // Símbolos Aritméticos e Numéricos
    Mais, // +
    Menos, // -
    Multiplicacao, // *
    Divisao, // /
    Modulo, // %
    Dolar, // $
    EComercial, // &


    Comentario,
}

#[derive(Debug)]
pub struct Token {
    pub tok: String,
    pub tipo: Tokens,
    pub lin: i32,
    pub col: i32,
}

fn TabelaSimbolos<'a>() -> Vec<Vec<String>> {
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
    let num_tokens = tabela.len(); // número de tokens no Vector

    let palavras_reservadas = vec!["and", "array", "asm", "begin", "boolean", "case", "char", "const", "div", "do", "else", "end", "false", "for", "function", "if", "integer", "not", "of", "or", "procedure", "program", "read", "string", "then", "to", "true", "type", "until", "var", "while", "with", "write"];

    let simbolos_pontuacao = vec!["(", ")", "[", "]", ":=", ".", ",", ";", ":", "..", "\'", "\""];

    let simbolos_relacao = vec!["=", "<>", "<", ">", "<=", ">="];

    let simbolos_aritmeticos = vec!["+", "-", "*", "/", "%", "$", "&"];

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
            if palavras_reservadas.contains(&(token.to_lowercase().as_str())){
                match token.to_lowercase().as_ref() {
                    "and" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::And, lin: linha, col: 0};
                    },
                    "array" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Array, lin: linha, col: 0};
                    },
                    "asm" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Asm, lin: linha, col: 0};
                    },
                    "begin" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Begin, lin: linha, col: 0};
                    },
                    "case" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Case, lin: linha, col: 0};
                    },
                    "const" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Const, lin: linha, col: 0};
                    },
                    "div" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Div, lin: linha, col: 0};
                    },
                    "do" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Do, lin: linha, col: 0};
                    },
                    "else" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Else, lin: linha, col: 0};
                    },
                    "end" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::End, lin: linha, col: 0};
                    },
                    "for" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::For, lin: linha, col: 0};
                    },
                    "function" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Function, lin: linha, col: 0};
                    },
                    "if" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::If, lin: linha, col: 0};
                    },
                    "not" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Not, lin: linha, col: 0};
                    },
                    "of" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Of, lin: linha, col: 0};
                    },
                    "or" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Or, lin: linha, col: 0};
                    },
                    "procedure" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Procedure, lin: linha, col: 0};
                    },
                    "program" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Program, lin: linha, col: 0};
                    },
                    "read" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Read, lin: linha, col: 0};
                    },
                    "string" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Reservada_String, lin: linha, col: 0};
                    },
                    "then" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Then, lin: linha, col: 0};
                    },
                    "to" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::To, lin: linha, col: 0};
                    },
                    "type" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Type, lin: linha, col: 0};
                    },
                    "until" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Until, lin: linha, col: 0};
                    },
                    "var" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Var, lin: linha, col: 0};
                    },
                    "while" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::While, lin: linha, col: 0};
                    },
                    "with" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::With, lin: linha, col: 0};
                    },
                    "write" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Write, lin: linha, col: 0};
                    },

                    _ => ()
                }
            } else if simbolos_pontuacao.contains(&(token.to_lowercase().as_str())){
                match token.as_ref() {
                    "(" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::AbreParenteses, lin: linha, col: 0};
                    },
                    ")" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::FechaParenteses, lin: linha, col: 0};
                    },
                    "[" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::AbreColchete, lin: linha, col: 0};
                    },
                    "]" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::FechaColchete, lin: linha, col: 0};
                    },
                    ":=" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Atribuicao, lin: linha, col: 0};
                    },
                    "." => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Ponto, lin: linha, col: 0};
                    },
                    "," => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Virgula, lin: linha, col: 0};
                    },
                    ";" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::PontoVirgula, lin: linha, col: 0};
                    },
                    ":" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::DoisPontos, lin: linha, col: 0};
                    },
                    ".." => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::DoisPontos, lin: linha, col: 0};
                    },
                    "\'" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Apostrofo, lin: linha, col: 0};
                    },
                    "\"" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Aspas, lin: linha, col: 0};
                    },

                    _ => {}
                }
            } else if simbolos_relacao.contains(&(token.to_lowercase().as_str())){
                match token.as_ref() {
                    "=" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Igual, lin: linha, col: 0};
                    },
                    "<>" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Diferente, lin: linha, col: 0};
                    },
                    "<" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Menor, lin: linha, col: 0};
                    },
                    ">" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Maior, lin: linha, col: 0};
                    },
                    "<=" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::MenorIgual, lin: linha, col: 0};
                    },
                    ">=" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::MaiorIgual, lin: linha, col: 0};
                    },

                    _ => ()
                }
                
            } else if simbolos_aritmeticos.contains(&(token.to_lowercase().as_str())){
                match token.as_ref() {
                    "+" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Mais, lin: linha, col: 0};
                    },
                    "-" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Menor, lin: linha, col: 0};
                    },
                    "*" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Multiplicacao, lin: linha, col: 0};
                    },
                    "/" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Divisao, lin: linha, col: 0};
                    },
                    "%" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Modulo, lin: linha, col: 0};
                    },
                    "$" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::Dolar, lin: linha, col: 0};
                    },
                    "&" => {
                        prox_token = Token{tok: token.to_string(), tipo: Tokens::EComercial, lin: linha, col: 0};
                    },

                    _ => ()
                }

            } else if token.to_string().parse::<i64>().is_ok() { // verifica se é número
                prox_token = Token{tok: token.to_string(), tipo: Tokens::Numero, lin: linha, col: 0};
            } else {
                //classifica como Identificador
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
        let aux = AnalisaLexico(&mut result[i], (i+1) as i32);

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
