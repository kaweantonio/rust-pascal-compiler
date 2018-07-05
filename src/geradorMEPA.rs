use lexer::Tokens;
use parser;
use parser::Simbolo;
use std::sync::Mutex;
use std::process::exit;

#[derive(Debug, PartialEq, Clone)]
pub struct Rotulo {
    pub tok: String,
    pub rotulo: String,
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
            rotulo: ["L", contador_rotulo.to_string().as_ref()].join(""),
            numero: contador_rotulo,
        };
        tabelaRotulos.lock().unwrap().push(rotulo);
        contador_rotulo = contador_rotulo + 1;
    }
}

fn procura_rotulo(token: String) -> String {
    let data = tabelaRotulos.lock().unwrap();

    for i in 0..data.len(){
        if data[i].tok == token {
            return data[i].rotulo.clone();
        }
    }

    return "".to_string();
}

fn procura_rotulo_numero(num: i32) -> String {
    let data = tabelaRotulos.lock().unwrap();

    for i in 0..data.len(){
        if data[i].numero == num {
            return data[i].rotulo.clone();
        }
    }

    return "".to_string();
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
    match token.to_lowercase().as_ref() {
        "+" => return "SOMA".to_string(),
        "-" => return "SUBT".to_string(),
        "*" => return "MULT".to_string(),
        "/" => return "DIVI".to_string(),
        ">" => return "CMMA".to_string(),
        ">=" => return "CMAG".to_string(),
        "<" => return "CMME".to_string(),
        "<=" => return "CMEG".to_string(),
        "=" => return "CMIG".to_string(),
        "<>" => return "CMDG".to_string(),
        "and" => return "CONJ".to_string(),
        _ => return "".to_string(),
    }
}

fn prioridade(a: String, b: String) -> bool {
    let mut pa = 0;
    let mut pb = 0;

    match a.to_lowercase().as_ref() {
        "(" => pa = 3,
        "*" | "/" => pa = 2,
        _ => pa = 1, // Tokens::Mais | Tokens::Menos | Tokens::Igual..Tokens::MaiorIgual | Tokens::And
    }

    match b.to_lowercase().as_ref() {
        "(" => pb = 0,
        "*" | "/" => pb = 2,
        "+" | "-" | "=" | "<" | "<=" | ">" | ">=" | "<>" | "and" => pb = 1,
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
            Tokens::If => if_statement(),
            Tokens::While => while_statement(),
            Tokens::Read => read_statement(),
            Tokens::Write => write_statement(),
            Tokens::For => for_statement(),
            Tokens::Repeat => repeat_statement(),
            _ => {},
        }

        simbolo = prox_simbolo();
        if simbolo.tipo == Tokens::End {
            break;
        }
    }
}

fn atribuicao(){
    let mut simbolo = prox_simbolo();
    let posicao = parser::procura_var(simbolo.tok);
    consome(); // identificador
    consome(); // símbolo atribuição

    if posicao >= 0 {
        let mut expressao = vec![prox_simbolo()];
        consome();

        'atribuicao: loop {
            simbolo = prox_simbolo();
            if simbolo.tipo == Tokens::Else {
                expressao.push(prox_simbolo());
                break 'atribuicao;
            }
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
            Tokens::True => {
                gera_mepa(["CRCT", "1"].join(" "));
            },
            Tokens::False => {
                gera_mepa(["CRCT", "0"].join(" "));
            }
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
            Tokens::FechaParenteses | Tokens::PontoVirgula | Tokens::Then | Tokens::Else | Tokens::Virgula | Tokens::Do => {
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
            Tokens::Mais | Tokens::Menos | Tokens::Multiplicacao | Tokens::Divisao | Tokens::Menor | Tokens:: MenorIgual | 
            Tokens::Maior | Tokens::MaiorIgual | Tokens::Igual | Tokens::Diferente | Tokens::And => {
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

fn if_statement() {
    let mut simbolo;
    let rotulo;
    let rotulo2;
    consome(); // consome if

    let mut expressao = vec![prox_simbolo()];
    consome();

    'if_loop: loop {
        simbolo = prox_simbolo();
        if simbolo.tipo == Tokens::Then {
            expressao.push(prox_simbolo());
            consome();
            break 'if_loop;
        }
        expressao.push(prox_simbolo());
        consome();
    }

    expressao_mepa(expressao);

    cria_rotulo("if".to_string());
    unsafe {
        rotulo = contador_rotulo - 1;
        gera_mepa(["DSVF", procura_rotulo_numero(rotulo).as_ref()].join(" "));
    }

    statement();
    
    simbolo = prox_simbolo();
    if simbolo.tipo == Tokens::Else {
        consome(); // consome else
        cria_rotulo("else".to_string());
        unsafe {
            rotulo2 = contador_rotulo - 1;
            gera_mepa(["DSVS", procura_rotulo_numero(rotulo2).as_ref()].join(" "));
            gera_mepa([procura_rotulo_numero(rotulo), "NADA".to_string()].join(" "));
        }

        statement();

        gera_mepa([procura_rotulo_numero(rotulo2), "NADA".to_string()].join(" "));
    } else {
        gera_mepa([procura_rotulo_numero(rotulo), "NADA".to_string()].join(" "));
    }
}

fn statement() {
    let mut simbolo = prox_simbolo();
    match simbolo.tipo {
        Tokens::Identificador => {
            unsafe {
                simbolo = parser::lookahead();
                match simbolo.tipo {
                    Tokens::AbreColchete | Tokens::Ponto | Tokens::Atribuicao => atribuicao(),
                    // Tokens::AbreParenteses | _ => procedure_call(),
                    _ => {},
                }
            }
        },
        Tokens::If => if_statement(),
        // Tokens::Case => case_statement(),
        Tokens::While => while_statement(),
        // Tokens::Repeat => repeat_statement(),
        Tokens::For => for_statement(),
        // Tokens::With => with_statement(),
        // Tokens::Goto => goto_statement(),
        Tokens::Begin => inicio_programa(),
        Tokens::Read => read_statement(),
        Tokens::Write => write_statement(),
        _ => {},
    }
}

fn while_statement(){
    // consome while
    consome();
    let mut simbolo;
    let mut rotulo1;
    let mut rotulo2;
    cria_rotulo("while".to_string());
    unsafe {
        rotulo1 = contador_rotulo;
        gera_mepa([procura_rotulo_numero(contador_rotulo-1), "NADA".to_string()].join(" "));
    }
    
    let mut expressao = vec![prox_simbolo()];
    consome();

    'while_loop: loop {
        simbolo = prox_simbolo();
        if simbolo.tipo == Tokens::Do {
            expressao.push(prox_simbolo());
            consome();
            break 'while_loop;
        }
        expressao.push(prox_simbolo());
        consome();
    }

    expressao_mepa(expressao);

    unsafe {
        cria_rotulo("do".to_string());
        rotulo2 = contador_rotulo;
        gera_mepa(["DSVF", procura_rotulo_numero(rotulo2-1).as_ref()].join(" "));
    }
    statement();

    unsafe {
        gera_mepa(["DSVS", procura_rotulo_numero(rotulo1-1).as_ref()].join(" "));
        gera_mepa([procura_rotulo_numero(rotulo2-1), "NADA".to_string()].join(" "));
    }
}

fn read_statement(){
    let mut simbolo;
    consome(); // consome read
    consome(); // consome '('
    'read_loop: loop {
        simbolo = prox_simbolo();

        match simbolo.tipo {
            Tokens::Identificador => {
                gera_mepa("LEIT".to_string());
                gera_mepa(["ARMZ", simbolo.tok.as_ref()].join(" "));
                consome();
            },
            Tokens::Virgula => consome(),
            _ => {
                consome(); // consome ')'
                break 'read_loop;
            }
        }
    }

    consome(); // consome ';'
}

fn write_statement(){
    let mut simbolo;
    consome(); // consome read
    consome(); // consome '('

    'write_outer_loop: loop {
        let mut expressao = vec![prox_simbolo()];
        consome();
        'write_inner_loop: loop {
            simbolo = prox_simbolo();
            if simbolo.tipo == Tokens::Virgula {
                expressao.push(prox_simbolo());
                consome();
                expressao_mepa(expressao);
                gera_mepa("IMPR".to_string());
                break 'write_inner_loop;
            } else if simbolo.tipo == Tokens::FechaParenteses {
                expressao.push(prox_simbolo());
                consome();
                expressao_mepa(expressao);
                gera_mepa("IMPR".to_string());
                break 'write_inner_loop;
            }
            expressao.push(prox_simbolo());
            consome();
        }
        
        simbolo = prox_simbolo();
        if simbolo.tipo == Tokens::AbreParenteses || simbolo.tipo == Tokens::PontoVirgula {
            break 'write_outer_loop;
        }
    }

    consome(); // consome ';'
}

fn for_statement(){
    let mut simbolo;
    let (rotulo, rotulo2);
    consome(); // consome for
    let var_i = prox_simbolo().tok;
    let mut passo;
    consome(); // consome var_i
    consome(); // consome :=

    let mut expressao = vec![prox_simbolo()];
    consome();

    'for_loop: loop {
        simbolo = prox_simbolo();
        if simbolo.tipo == Tokens::To {
            passo = Tokens::To;
            expressao.push(prox_simbolo());
            consome();
            break 'for_loop;
        } else if simbolo.tipo == Tokens::Downto {
            passo = Tokens::To;
            expressao.push(prox_simbolo());
            consome();
            break 'for_loop;
        }
        expressao.push(prox_simbolo());
        consome();
    }

    expressao_mepa(expressao);

    gera_mepa(["AMRZ", var_i.to_string().as_ref()].join(" "));
    cria_rotulo("for".to_string());
    unsafe {
        rotulo = contador_rotulo-1;
        gera_mepa([procura_rotulo_numero(rotulo).as_ref(), "NADA"].join(" "));
    }

    gera_mepa(["CRVL", parser::procura_var(var_i.clone()).to_string().as_ref()].join(" "));

    expressao = vec![prox_simbolo()];
    consome();

    'for_loop2: loop {    
        simbolo = prox_simbolo();
        if simbolo.tipo == Tokens::Do {
            expressao.push(prox_simbolo());
            consome();
            break 'for_loop2;
        }
        expressao.push(prox_simbolo());
        consome();
    }

    expressao_mepa(expressao);

    if passo == Tokens::To {
        gera_mepa("CMEG".to_string());
    } else {
        gera_mepa("CMAG".to_string());
    }

    cria_rotulo("do".to_string());
    unsafe {
        rotulo2 = contador_rotulo-1;
    }

    gera_mepa(["DSVF", procura_rotulo_numero(rotulo2).as_ref()].join(" "));

    statement();

    gera_mepa(["CRVL", parser::procura_var(var_i.clone()).to_string().as_ref()].join(" "));
    gera_mepa("CRCT 1".to_string());

    if passo == Tokens::To {
        gera_mepa("SOMA".to_string());
    } else {
        gera_mepa("SUBT".to_string());
    }

    gera_mepa(["ARMZ", parser::procura_var(var_i).to_string().as_ref()].join(" "));
    gera_mepa(["DSVS", procura_rotulo_numero(rotulo).as_ref()].join(" "));
    gera_mepa([procura_rotulo_numero(rotulo2).as_ref(), "NADA"].join(" "));
}

fn repeat_statement(){
    consome(); // consome repeat
    let rotulo;
    let mut simbolo;
    
    cria_rotulo("repeat".to_string());

    unsafe {
        rotulo = contador_rotulo - 1;
        gera_mepa([procura_rotulo_numero(rotulo), "NADA".to_string()].join("  "));
    }

    statement();
    simbolo = prox_simbolo();
    while prox_simbolo().tipo != Tokens::Until {
        statement();
    }

    consome(); // consome until

    let mut expressao = vec![prox_simbolo()];
    consome();
    
    'repeat_statement_inner_loop: loop {
        simbolo = prox_simbolo();
        if simbolo.tipo == Tokens::PontoVirgula {
            expressao.push(prox_simbolo());
            consome();
            break 'repeat_statement_inner_loop;
        }

        expressao.push(prox_simbolo());
        consome();
    }
    
    expressao_mepa(expressao);

    gera_mepa(["DSVF", procura_rotulo_numero(rotulo).as_ref()].join(" "));

    cria_rotulo("until".to_string());
    unsafe {
        let rotulo2 = contador_rotulo - 1;
        gera_mepa([procura_rotulo_numero(rotulo2), "NADA".to_string()].join(" "));
    }
}
