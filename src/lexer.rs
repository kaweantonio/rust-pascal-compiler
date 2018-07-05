// use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::sync::Mutex;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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
    Downto,
    Else,
    End,
    False,
    File,
    For,
    Forward,
    Function,
    Goto,
    If,
    In,
    Inline,
    Integer,
    Label,
    Mod,
    Nil,
    Not,
    Object,
    Of,
    Or,
    Packed,
    Procedure,
    Program,
    Read,
    Real,
    Record,
    Repeat,
    Set,
    ReservadaString,
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
    FatorEscala, // E | e


    Comentario,
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Tokens::And => "And",
            Tokens::Array => "Array",
            Tokens::Asm => "Asm",
            Tokens::Begin => "Begin",
            Tokens::Boolean => "Boolean",
            Tokens::Case => "Case",
            Tokens::Char => "Char",
            Tokens::Const => "Const",
            Tokens::Div => "Div",
            Tokens::Do => "Do",
            Tokens::Downto => "Downto",
            Tokens::Else => "Else",
            Tokens::End => "End",
            Tokens::False => "False",
            Tokens::File => "File",
            Tokens::For => "For",
            Tokens::Forward => "Forward",
            Tokens::Function => "Function",
            Tokens::Goto => "Goto",
            Tokens::If => "If",
            Tokens::In => "In",
            Tokens::Inline => "Inline",
            Tokens::Integer => "Integer",
            Tokens::Label => "Label",
            Tokens::Mod => "Mod",
            Tokens::Nil => "Nil",
            Tokens::Not => "Not",
            Tokens::Object => "Object",
            Tokens::Of => "Of",
            Tokens::Or => "Or",
            Tokens::Packed => "Packed",
            Tokens::Procedure => "Procedure",
            Tokens::Program => "Program",
            Tokens::Read => "Read",
            Tokens::Real => "Real",
            Tokens::Record => "Record",
            Tokens::Repeat => "Repeat",
            Tokens::Set => "Set",
            Tokens::ReservadaString => "String",
            Tokens::Then => "Then",
            Tokens::To => "To",
            Tokens::True => "True",
            Tokens::Type => "Type",
            Tokens::Until => "Until",
            Tokens::Var => "Var",
            Tokens::While => "While",
            Tokens::With => "With",
            Tokens::Write => "Write",
            Tokens::Identificador => "Identificador",
            Tokens::Numero => "Numero",
            Tokens::AbreParenteses => "(",
            Tokens::FechaParenteses => ")",
            Tokens::AbreColchete => "[",
            Tokens::FechaColchete => "]",
            Tokens::Atribuicao => ":=",
            Tokens::Ponto => ".",
            Tokens::Virgula => ",",
            Tokens::PontoVirgula => ";",
            Tokens::DoisPontos => ":",
            Tokens::Apostrofo => "\'",
            Tokens::Aspas => "\"",
            Tokens::Igual => "=",
            Tokens::Diferente => "<>",
            Tokens::Menor => "<",
            Tokens::Maior => ">",
            Tokens::MenorIgual => "<=",
            Tokens::MaiorIgual => ">=",
            Tokens::Mais => "+",
            Tokens::Menos => "-",
            Tokens::Multiplicacao => "*",
            Tokens::Divisao => "/",
            Tokens::Modulo => "%",
            Tokens::Dolar => "$",
            Tokens::EComercial => "&",
            Tokens::FatorEscala => "E ou e",
            Tokens::Comentario => "",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub tok: String,
    pub tipo: Tokens,
    pub lin: i32,
    pub col: i32,
}

impl Token {
    pub fn new() -> Token {
        Token {
            tok: ("").to_string(),
            tipo: Tokens::Comentario,
            lin: 0,
            col: 0
        }
    }
}

static PALAVRAS_RESERVADAS: &'static [&'static str] = &["and", "array", "asm", "begin", "boolean", "case", "char", "const", "div", "do", "downto", "else", "end", "false", "file", "for", "forward", "function", "goto", "if", "in", "inline", "integer", "label","mod", "nil", "not", "object", "of", "or", "packed", "procedure", "program", "read", "real", "record", "repeat", "set", "string", "then", "to", "true", "type", "until", "var", "while", "with", "write"];

static SIMBOLOS_PONTUACAO: &'static [&'static str] = &["(", ")", "[", "]", ".", ",", ";", ":", "\'", "\""];
static SIMBOLOS_RELACAO: &'static [&'static str] = &["=", "<", ">"];

static SIMBOLOS_ARITMETICOS: &'static [&'static str] = &["+", "-", "*", "/", "%", "$", "&"];

lazy_static! {
    pub static ref tabelaToken: Mutex<Vec<Vec<Token>>> = Mutex::new(vec![Vec::<Token>::new()]);
}

fn tabelaSimbolos<'a>() -> Vec<Vec<String>> {
    // let arquivo: String = read!("{}\r\n");
    // print!("{:?}", arquivo);
    let mut file = File::open("src/input.txt").expect("Não foi possível abrir o arquivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Não foi possível ler o arquivo");

    let mut aux = Vec::new();
    aux.push(Vec::<String>::new());
    let mut i = 0;
    let mut j = 0;

    for (indice, caractere) in contents.match_indices(|c: char| !(c.is_alphanumeric() || !(c != '_'))) {
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

fn analisaLexico(tabela:&mut Vec<String>, linha: i32) -> Vec<Token> {
    let num_tokens = tabela.len(); // número de tokens no Vector

    let mut prox_token;
    let mut aux = Vec::<Token>::new();

    for token in tabela{
        prox_token = Token {
            tok: ("").to_string(),
            tipo: Tokens::Comentario,
            lin: 0,
            col: 0
        };

        // verifica se é reservada
        if PALAVRAS_RESERVADAS.contains(&(token.to_lowercase().as_str())){
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
                "boolean" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Boolean, lin: linha, col: 0};
                },
                "case" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Case, lin: linha, col: 0};
                },
                "char" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Char, lin: linha, col: 0};
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
                "downto" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Downto, lin: linha, col: 0};
                },
                "else" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Else, lin: linha, col: 0};
                },
                "end" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::End, lin: linha, col: 0};
                },
                "false" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::False, lin: linha, col: 0};
                },
                "file" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::File, lin: linha, col: 0};
                },
                "for" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::For, lin: linha, col: 0};
                },
                "forward" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Forward, lin: linha, col: 0};
                },
                "function" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Function, lin: linha, col: 0};
                },
                "goto" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Goto, lin: linha, col: 0};
                },
                "if" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::If, lin: linha, col: 0};
                },
                "in" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::In, lin: linha, col: 0};
                },
                "inline" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Inline, lin: linha, col: 0};
                },
                "integer" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Integer, lin: linha, col: 0};
                },
                "label" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Label, lin: linha, col: 0};
                },
                "mod" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Mod, lin: linha, col: 0};
                },
                "nil" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Nil, lin: linha, col: 0};
                },
                "not" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Not, lin: linha, col: 0};
                },
                "object" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Object, lin: linha, col: 0};
                },
                "of" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Of, lin: linha, col: 0};
                },
                "or" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Or, lin: linha, col: 0};
                },
                "packed" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Packed, lin: linha, col: 0};
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
                "real" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Real, lin: linha, col: 0};
                },
                "record" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Record, lin: linha, col: 0};
                },
                "repeat" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Repeat, lin: linha, col: 0};
                },
                "set" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Set, lin: linha, col: 0};
                },
                "string" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::ReservadaString, lin: linha, col: 0};
                },
                "then" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Then, lin: linha, col: 0};
                },
                "to" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::To, lin: linha, col: 0};
                },
                "true" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::True, lin: linha, col: 0};
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
        } else if SIMBOLOS_PONTUACAO.contains(&(token.to_lowercase().as_str())){
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
        } else if SIMBOLOS_RELACAO.contains(&(token.to_lowercase().as_str())){
            match token.as_ref() {
                "=" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Igual, lin: linha, col: 0};
                },
                "<" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Menor, lin: linha, col: 0};
                },
                ">" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Maior, lin: linha, col: 0};
                },
                _ => ()
            }
            
        } else if SIMBOLOS_ARITMETICOS.contains(&(token.to_lowercase().as_str())){
            match token.as_ref() {
                "+" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Mais, lin: linha, col: 0};
                },
                "-" => {
                    prox_token = Token{tok: token.to_string(), tipo: Tokens::Menos, lin: linha, col: 0};
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
        } else if token.to_lowercase() == "e"{
            prox_token = Token{tok: token.to_string(), tipo: Tokens::FatorEscala, lin: linha, col: 0};
        } else {
            //classifica como Identificador
            prox_token = Token{tok: token.to_string(), tipo: Tokens::Identificador, lin: linha, col: 0};
        }
        aux.push(prox_token);
    }

    return aux;
}

fn tratamento(mut vec_tok: Vec<Token>) -> Vec<Token>{
    let mut tamanho = vec_tok.len()-1;
    let mut i = 0;

    while i < tamanho {
        if vec_tok[i].tipo == Tokens::DoisPontos && vec_tok[i+1].tipo == Tokens::Igual{
            vec_tok[i].tipo = Tokens::Atribuicao;
            vec_tok[i].tok = ":=".to_string();
            vec_tok.remove(i+1);
            tamanho = tamanho-1;
        } else if vec_tok[i].tipo == Tokens::Menor && vec_tok[i+1].tipo == Tokens::Maior {
            vec_tok[i].tipo = Tokens::Diferente;
            vec_tok[i].tok = "<>".to_string();
            vec_tok.remove(i+1);
            tamanho = tamanho-1;
        } else if vec_tok[i].tipo == Tokens::Menor && vec_tok[i+1].tipo == Tokens::Igual {
            vec_tok[i].tipo = Tokens::MenorIgual;
            vec_tok[i].tok = "<=".to_string();
            vec_tok.remove(i+1);
            tamanho = tamanho-1;
        } else if vec_tok[i].tipo == Tokens::Maior && vec_tok[i+1].tipo == Tokens::Igual {
            vec_tok[i].tipo = Tokens::MaiorIgual;
            vec_tok[i].tok = ">=".to_string();
            vec_tok.remove(i+1);
            tamanho = tamanho-1;
        } else if vec_tok[i].tipo == Tokens::Divisao && vec_tok[i+1].tipo == Tokens::Divisao {
            vec_tok.split_off(i);
            return vec_tok;
        } else if vec_tok[i].tipo == Tokens::FatorEscala {
            if i > 0 {
                if vec_tok[i-1].tipo != Tokens::Ponto {
                    vec_tok[i].tipo = Tokens::Identificador;
                } 
            } else {
                vec_tok[i].tipo = Tokens::Identificador;
            }
        } else if vec_tok[i].tipo == Tokens::Numero {
            if vec_tok[i+1].tipo == Tokens::Ponto {
                if vec_tok[i+2].tipo == Tokens::Numero {
                    let mut aux: String = vec_tok[i].tok.to_owned();
                    let str1: String = vec_tok[i+1].tok.to_owned();
                    let str2: String = vec_tok[i+2].tok.to_owned();

                    aux.push_str(&str1);
                    aux.push_str(&str2);

                    vec_tok[i].tok = aux.to_string();
                    vec_tok.remove(i+1);
                    vec_tok.remove(i+1);
                    tamanho = tamanho - 2;
                }
            }
        }
        i = i+1;
    }

    i = 0;
    while i < tamanho {
        if vec_tok[i].tipo == Tokens::Aspas || vec_tok[i].tipo == Tokens::Apostrofo {
            let mut j = i + 1;
            while vec_tok[j].tipo != Tokens::Aspas && vec_tok[j].tipo != Tokens::Apostrofo {
                vec_tok[j].tipo = Tokens::Identificador;
                j = j + 1;
            }
            i = j;
        }
        i = i + 1;
    }

    return vec_tok;
}

pub fn lexico() {
    let mut result = tabelaSimbolos();
    let mut aux;
    let mut aux2;

    println!("{:?}",result);
    println!("\n\n");

    for i in 0..result.len(){
        aux = analisaLexico(&mut result[i], (i+1) as i32);
        
        aux2 = tratamento(aux);

        if aux2.len() > 0 {
            tabelaToken.lock().unwrap().push(aux2);
        }
    }

    tabelaToken.lock().unwrap().remove(0);

    println!("\n\n");

    unsafe {
        let data = tabelaToken.lock().unwrap();
        for i in 0..data.len(){
            println!("Linha: {0}", i+1);
            for j in 0..data[i].len(){
                println!("{:?}", data[i][j]);
            }
        }
    }
}

pub fn hasToken(linha: usize) -> bool {
    return !tabelaToken.lock().unwrap()[linha].is_empty();
}

pub fn getToken(linha: usize) -> Token{
    return tabelaToken.lock().unwrap()[linha][0].clone();
}

pub fn eraseToken(linha: usize){
    tabelaToken.lock().unwrap()[linha].remove(0);
}

pub fn lookahead_nextline(linha: usize) -> Token {
    if size(linha) > 1 {
        return tabelaToken.lock().unwrap()[linha][0].clone();
    } else {
        return Token {
            tok: ("").to_string(),
            tipo: Tokens::Comentario,
            lin: 0,
            col: 0
        };   
    }
}

pub fn lookahead(linha: usize) -> Token {
    if size(linha) > 1 {
        return tabelaToken.lock().unwrap()[linha][1].clone();
    } else {
        return Token {
            tok: ("").to_string(),
            tipo: Tokens::Comentario,
            lin: 0,
            col: 0
        };   
    }
}

pub fn size(linha: usize) -> usize {
    return tabelaToken.lock().unwrap()[linha].len();
}
