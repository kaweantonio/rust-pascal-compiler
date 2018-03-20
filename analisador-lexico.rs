use std::*;
use std::fs::File;
use std::io::prelude::*;

// Declarando conjunto de palavras reservadas

const KEYWORDS: &'static [&'static str; 32] = &["AND", "ARRAY", "BEGIN", "CASE", "CONST", 
"DIV", "DO", "DOWNTO", "ELSE", "END", "FOR", "FUNCTION", "GOTO", "IF", "LABEL", "MOD", 
"NOT", "OF", "OR", "POINTER", "PROCEDURE", "PROGRAM", "RECORD", "REPEAT", "SET", "THEN", 
"TO", "TYPE", "UNTIL", "VAR", "WHILE", "WITH"];

fn cria_tabela_de_simbolos() -> Vec<String>{
	let vec: Vec<String> = Vec::with_capacity(1000);
	return vec;
}

fn abre_arquivo(filename: &str){
	let st = cria_tabela_de_simbolos();

	let mut f = File::open(filename).expect("file not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
      .expect("something went wrong reading the file");


  let v : Vec<String>;
  v = contents.split_whitespace().map(String::from).collect();
  println!("{:?}", v);
}

fn main(){
	let f = "programa-pascal-exemplo.txt".to_string();
	abre_arquivo(f.as_ref());
}