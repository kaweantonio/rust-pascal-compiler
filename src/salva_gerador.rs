use lexer::Tokens;
use parser;
use parser::Simbolo;
use std::sync::Mutex;
use std::process::exit;

#[derive(Debug, PartialEq, Clone)]
pub struct Rotulo {
    pub tok: String,
    pub numero: i32,
}

lazy_static! {
    pub static ref codigoMEPA: Mutex<Vec<String>> = Mutex::new(Vec::<String>::new());
    pub static ref tabelaRotulos: Mutex<Vec<Rotulo>> = Mutex::new(Vec::<Rotulo>::new());
    pub static ref pilha_pol: Mutex<Vec<String>> = Mutex::new(Vec::<String>::new());
}

static mut contador_rotulo: i32 = 0;
static mut escopo: &'static str = "";

fn erro_msg(msg: String){
    println!("{}", msg);
    exit(1);

}

fn prox_simbolo() -> parser::Simbolo {
    unsafe {
        if (!parser::hasSymbol()){
            let string = ["ERRO na GERAÇÃO MEPA: Fim inesperado do arquivo."].join("\n");
            erro_msg(string);
        }
        return parser::getSymbol();
    }
}

fn consome(){
    parser::eraseSymbol();
}

fn gera_mepa(codigo: String){
    codigoMEPA.lock().unwrap().push(codigo);
}

fn cria_rotulo(token: String){
    unsafe {
        let rotulo = Rotulo {
            tok: token,
            numero: contador_rotulo,
        };

        gera_mepa(["R", contador_rotulo.to_string().as_ref()].join(""));
        tabelaRotulos.lock().unwrap().push(rotulo);
        contador_rotulo = contador_rotulo + 1;
    }
}

fn empilha(x: String){
    pilha_pol.lock().unwrap().push(x);
}

fn desempilha() -> String {
    let x = pilha_pol.lock().unwrap().pop();
    match x {
        Some(_) => return x.unwrap(),
        None => return "None".to_string()
    }
}

fn instrucao(token: String) -> String {
    match token.as_ref() {
        "+" => return "SOMA".to_string(),
        "-" => return "SUB".to_string(),
        "*" => return "MULT".to_string(),
        "/" => return "DIV".to_string(),
        ">" => return "CMMA".to_string(),
        ">=" => return "CMAG".to_string(),
        "<" => return "CMME".to_string(),
        "<=" => return "CMEG".to_string(),
        "=" => return "CMIG".to_string(),
        _ => return "".to_string(),
    }
}

fn prioridade(a: String, b: String) -> bool {
    let mut pa = 0;
    let mut pb = 0;

    match a.as_ref() {
        "(" => pa = 3,
        "*" | "/" => pa = 2,
        _ => pa = 1, // Tokens::Mais | Tokens::Menos | Tokens::Igual..Tokens::MaiorIgual
    }

    match b.as_ref() {
        "(" => pb = 0,
        "*" | "/" => pb = 2,
        "+" | "-" | "=" | "<" | "<=" | ">" | ">=" => pb = 1,
        _ => pb = 0,
    }

    return pa > pb;
}

pub fn MEPA(){
    parser::ASD();

    inicio_leitura();

    print!("\n");

    let data = codigoMEPA.lock().unwrap();
    for i in 0..data.len(){
        println!("{}", data[i]);
    }

    let data2 = pilha_pol.lock().unwrap();
    for i in 0..data2.len(){
        println!("{}", data2[i]);
    }
}

fn inicio_leitura(){
    // program
    gera_mepa("INPP".to_string());

    while prox_simbolo().tipo != Tokens::Begin { 
        consome();
    }

    // var_declaration
    consome();

    let mut data = parser::codigoAMEM.lock().unwrap();
    for i in 0..data.len(){
        gera_mepa(data.remove(0));
    }

    inicio_programa();
}

fn inicio_programa(){
    let mut simbolo = prox_simbolo();

    if simbolo.tipo == Tokens::Begin {
        consome();
    }

    simbolo = prox_simbolo();
    loop {
        match simbolo.tipo {
            Tokens::Identificador => atribuicao(),
            _ => {},
        }

        simbolo = prox_simbolo();
        if simbolo.tipo == Tokens::End {
            break;
        }
    }
}

fn atribuicao(){
    let mut simbolo = parser::lookahead();

    let posicao = parser::procura_var(prox_simbolo().tok);
    consome(); // identificador
    consome(); // símbolo atribuição

    if posicao >= 0 {
        let mut expressao = vec![prox_simbolo()];
        consome();

        'atribuicao: loop {
            simbolo = prox_simbolo();
            if simbolo.tipo == Tokens::PontoVirgula {
                expressao.push(prox_simbolo());
                consome();
                break 'atribuicao;
            }
            expressao.push(prox_simbolo());
            consome();
        }

        expressao_mepa(expressao);

        simbolo = prox_simbolo();
        if simbolo.tipo == Tokens::PontoVirgula {
            consome();

        }

        gera_mepa(["ARMZ", posicao.to_string().as_ref()].join(" "));
    }
}

fn expressao_mepa(expr: Vec<Simbolo>){
    let mut i = 0;
    let mut simbolo;
    let mut posicao;
    empilha('('.to_string());

    while i < expr.len() {
        simbolo = &expr[i];

        let salva = simbolo.clone();

        match simbolo.tipo {
            Tokens::Numero => {
                let string = &expr[i].tok.to_owned();

                gera_mepa(["CRCT", string.as_ref()].join(" "));
            },
            Tokens::Identificador => {
                posicao = parser::procura_var(salva.tok);
                gera_mepa(["CRVL", posicao.to_string().as_ref()].join(" "));
            },
            Tokens::AbreParenteses => {
                empilha('('.to_string());
            },
            Tokens::FechaParenteses | Tokens::PontoVirgula | Tokens::Then | Tokens::Virgula | Tokens::Do => {
                loop {
                    let aux = desempilha();

                    if aux.clone() != '('.to_string() {
                        let inst = instrucao(aux);
                        gera_mepa(inst);
                    } else {
                        break;
                    }
                }
            },
            Tokens::Mais | Tokens::Menos | Tokens::Multiplicacao | Tokens::Div | Tokens::Menor | Tokens:: MenorIgual | 
            Tokens::Maior | Tokens::MaiorIgual | Tokens::Igual => {
                let simbolo = salva.clone();
                let mut aux;
                loop {
                    aux = desempilha();
                    let aux2 = aux.clone();
                    if prioridade(simbolo.tok.clone(), aux) {
                        empilha(aux2);
                        empilha(simbolo.tok);

                        break;
                    } else {
                        gera_mepa(instrucao(aux2));
                    }
                }
            },
            _ => {},
        }

        i = i + 1;
    }
}
