use std::*;
use std::fs::File;
use std::io::prelude::*;

// Declarando conjunto de palavras reservadas

const KEYWORDS: &'static [&'static str; 32] = &["AND", "ARRAY", "BEGIN", "CASE", "CONST", 
"DIV", "DO", "DOWNTO", "ELSE", "END", "FOR", "FUNCTION", "GOTO", "IF", "LABEL", "MOD", 
"NOT", "OF", "OR", "POINTER", "PROCEDURE", "PROGRAM", "RECORD", "REPEAT", "SET", "THEN", 
"TO", "TYPE", "UNTIL", "VAR", "WHILE", "WITH"];

// Vector de símbolos. Irá salvar os identificadores encontrados
fn cria_tabela_de_simbolos() -> Vec<String>{
	let vec: Vec<String> = Vec::with_capacity(1000);
	return vec;
}

fn match_char(data: &char) -> bool {
    match *data {
        '\x01'...'\x08' |
        '\u{10FFFE}'...'\u{10FFFF}' => true,
        _ => false,
    }
}

// abre arquivo contendo o programa-fonte em código-fonte Pascal
fn abre_arquivo(filename: &str){ 
	let mut f = File::open(filename).expect("file not found");

  let mut contents = String::new();

  f.read_to_string(&mut contents).expect("something went wrong reading the file");

  let v: Vec<String> = contents.split_whitespace().map(String::from).collect();

  let mut identificadores = cria_tabela_de_simbolos();
  let mut significados = cria_tabela_de_simbolos();

 	// for i in 0..v.len(){
 	// 	let t = v[i].len();

 	// 	for j in 0..t {
 	// 		if (v[i].chars().next().unwrap() < '\x30')
 	// 	}
 	// }

  println!("{}", '\u{10FFFE}');
  // for i in 0..v.len(){
  // 	st.push(v[i].to_string());
  // }

  // println!("{:?}", st);
}

fn main(){
	let f = "programa-pascal-exemplo.txt".to_string();
	abre_arquivo(f.as_ref());
}