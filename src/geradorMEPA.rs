use lexer::Tokens;
use parser;
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

fn erro(simbolo_lido: parser::Simbolo, simbolo_esperado: Tokens){
    println!("ERRO na GERAÇAO MEPA: Esperado simbolo {0}, mas encontrou {1}", simbolo_esperado, simbolo_lido.tipo);
    
    unsafe {
        print!("{:?}", parser::tabelaSimb.lock().unwrap());
    }
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

fn consome(simbolo_atual: parser::Simbolo, simbolo_esperado: Tokens){
    if simbolo_atual.tipo != simbolo_esperado {
        erro(simbolo_atual, simbolo_esperado);
    }
    else {
        parser::eraseSymbol();
    }
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

fn instrucao(token: Tokens) -> String {
    match token {
        Tokens::Mais => return "SOMA".to_string(),
        Tokens::Menos => return "SUB".to_string(),
        Tokens::Multiplicacao => return "MULT".to_string(),
        Tokens::Divisao => return "DIV".to_string(),
        Tokens::Maior => return "CMMA".to_string(),
        Tokens::MaiorIgual => return "CMAG".to_string(),
        Tokens::Menor => return "CMME".to_string(),
        Tokens::MenorIgual => return "CMEG".to_string(),
        Tokens::Igual => return "CMIG".to_string(),
        _ => return "".to_string(),
    }
}

pub fn MEPA(){
    parser::ASD();

    empilha('('.to_string());

    program();

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

// program ::= program identifier [ ( identifier_list ) ] ; block .
fn program(){
    let mut simbolo = prox_simbolo();
    consome(simbolo, Tokens::Program);

    gera_mepa("INPP".to_string());

    identifier();
    simbolo = prox_simbolo();

    if (simbolo.tipo == Tokens::AbreParenteses){
        consome(simbolo, Tokens::AbreParenteses);
        identifier_list();
        simbolo = prox_simbolo();
        consome(simbolo, Tokens::FechaParenteses);
        simbolo = prox_simbolo();
    }

    consome(simbolo, Tokens::PontoVirgula);
    
    unsafe {
        escopo = "program";
    }

    block();

    simbolo = prox_simbolo();
    consome(simbolo, Tokens::Ponto);
}

/*
block ::= { [label_declaration_part]
            [const_declaration_part]
            [type_declaration_part]
            [var_declaration_part]
            [subroutine_declaration_part] }
            compound_statement
*/
fn block(){
    let mut simbolo = prox_simbolo();
    // if simbolo.tipo == Tokens::Label {
    //     label_declaration_part();
    //     simbolo = prox_simbolo();
    // }

    // if simbolo.tipo == Tokens::Const{
    //     const_declaration_part();
    //     simbolo = prox_simbolo();
    // }

    // if simbolo.tipo == Tokens::Type{
    //     type_declaration_part();
    //     simbolo = prox_simbolo();
    // }

    if simbolo.tipo == Tokens::Var{
        var_declaration_part();
        simbolo = prox_simbolo();
    }

    // if simbolo.tipo == Tokens::Procedure || simbolo.tipo == Tokens::Function {
    //      subroutine_declaration_part();
    // }
    
    compound_statement();
}

// identifier_list ::= identifier { , identifier }
fn identifier_list(){
    let mut simbolo;

    identifier();
    simbolo = prox_simbolo();
    while simbolo.tipo == Tokens::Virgula {
        consome(simbolo, Tokens::Virgula);
        identifier();
        simbolo = prox_simbolo();
    }
}

fn identifier(){
    let simbolo = prox_simbolo();
    let aux = prox_simbolo();
    consome(simbolo, Tokens::Identificador);
}

fn ehString() -> bool {
    let simbolo = prox_simbolo();
    return simbolo.tipo == Tokens::Apostrofo || simbolo.tipo == Tokens::Aspas;
}

fn string(){
    // sinaliza se string abriu com aspas [true] ou apóstrofo [false]
    let mut tipo = false;
    let mut simbolo = prox_simbolo();
    if simbolo.tipo == Tokens::Apostrofo {
        consome(simbolo, Tokens::Apostrofo);
    } else {
        tipo = true;
        consome(simbolo, Tokens::Aspas);
    }

    simbolo = prox_simbolo();
    while (simbolo.tipo != Tokens::Apostrofo && simbolo.tipo != Tokens::Aspas){
        identifier();
        simbolo = prox_simbolo();
    }
    if(!tipo){
        consome(simbolo, Tokens::Apostrofo);
    } else {
        consome(simbolo, Tokens::Aspas);
    }
}

fn ehSequenciaDigitos() -> bool {
    unsafe {
        let aux = parser::lookahead();
        return aux.tipo != Tokens::Ponto && aux.tipo != Tokens::FatorEscala;
    }
}

fn number_list(){
    number();

    let mut simbolo = prox_simbolo();

    while simbolo.tipo == Tokens::Virgula {
        consome(simbolo, Tokens::Virgula);
        number();
        simbolo = prox_simbolo();
    }
}

fn number() {
    let mut simbolo = prox_simbolo();
    let mut aux = prox_simbolo().tok;

    consome(simbolo, Tokens::Numero);

    simbolo = prox_simbolo();
    unsafe {
        if simbolo.tipo == Tokens::Ponto && parser::lookahead().tipo == Tokens::Numero {
            aux = format!("{}{}", aux, prox_simbolo().tok);
            consome(simbolo, Tokens::Ponto);
            simbolo = prox_simbolo();
            aux = format!("{}{}", aux, prox_simbolo().tok);
            consome(simbolo, Tokens::Numero);
            simbolo = prox_simbolo();
        }
    }

    empilha(aux);
}

fn temSinal() -> bool {
    let simbolo = prox_simbolo();
    return simbolo.tipo == Tokens::Mais || simbolo.tipo == Tokens::Menos;
}

fn sinal(){
    let simbolo = prox_simbolo();
    if simbolo.tipo == Tokens::Mais {
        empilha(instrucao(Tokens::Mais));
        consome(simbolo, Tokens::Mais);
    } else {
        empilha(instrucao(Tokens::Menos));
        consome(simbolo, Tokens::Menos);
    }
}

// var_declaration_part ::= var var_declaration { ; var_declaration} ;
fn var_declaration_part(){
    let mut simbolo = prox_simbolo();
    consome(simbolo, Tokens::Var);
    var_declaration();

    gera_mepa(parser::codigoAMEM.lock().unwrap().remove(0));

    simbolo = prox_simbolo();
    unsafe {
        while simbolo.tipo == Tokens::PontoVirgula && parser::lookahead().tipo == Tokens::Identificador{
            consome(simbolo, Tokens::PontoVirgula);
            var_declaration();
            gera_mepa(parser::codigoAMEM.lock().unwrap().remove(0));
            simbolo = prox_simbolo();
        }
    }

    consome(simbolo, Tokens::PontoVirgula);
}

// var_declaration ::= identifier_list_var : type
fn var_declaration(){
    let mut simbolo;
    identifier_list();

    simbolo = prox_simbolo();

    consome(simbolo, Tokens::DoisPontos);
    simbolo = prox_simbolo();

    match simbolo.tipo {
        Tokens::Integer => consome(simbolo, Tokens::Integer),
        Tokens::Real => consome(simbolo, Tokens::Real),
        Tokens::Char => consome(simbolo, Tokens::Char),
        Tokens::Boolean => consome(simbolo, Tokens::Boolean),
        _ => {
            let string = ["Erro: Esperado algum tipo de variável básico [Integer, Real, Char, ou Boolean] mas foi encontrado", simbolo.tok.as_ref()].join("\n");
            erro_msg(string);
        },
    }
}

// compound_statement ::= begin labeled_statement { ; labeled_statement  } end
fn compound_statement() {
    let mut simbolo = prox_simbolo();
    consome(simbolo, Tokens::Begin);

    

    unsafe {
        cria_rotulo(escopo.to_string());
    }

    labeled_statement();
    simbolo = prox_simbolo();

    'compound_statement_loop: loop {
        if simbolo.tipo != Tokens::PontoVirgula {
            break 'compound_statement_loop;
        }

        consome(simbolo, Tokens::PontoVirgula);
        labeled_statement();
        simbolo = prox_simbolo();
    }

    consome(simbolo, Tokens::End);
}

// labeled_statement ::= [ number :] statement   
fn labeled_statement() {
    let mut simbolo = prox_simbolo();

    if simbolo.tipo == Tokens::Numero {
        consome(simbolo, Tokens::Numero);
        simbolo = prox_simbolo();
        consome(simbolo, Tokens::DoisPontos);
    }

    statement(); 
}

/*
statement ::= assign_statement
| procedure_call
| if_statement
| case_statement
| while_statement
| repeat_statement
| for_statement
| with_statement
| goto_statement
| compound_statement
| Vazio */
fn statement() {
    let mut simbolo = prox_simbolo();
    match simbolo.tipo {
        Tokens::Identificador => {
            unsafe {
                simbolo = parser::lookahead();
                match simbolo.tipo {
                    Tokens::AbreColchete | Tokens::Ponto | Tokens::Atribuicao => assign_statement(),
                    // Tokens::AbreParenteses | _ => procedure_call(),
                    _ => {},
                }
            }
        },
        // Tokens::If => if_statement(),
        // Tokens::Case => case_statement(),
        // Tokens::While => while_statement(),
        // Tokens::Repeat => repeat_statement(),
        // Tokens::For => for_statement(),
        // Tokens::With => with_statement(),
        // Tokens::Goto => goto_statement(),
        // Tokens::Begin => compound_statement(),
        // _ => internal_functions(),
        _ => {},
    }
}

// assign_statement ::= identifier [ infipo ] := expr
fn assign_statement(){
    let mut simbolo;

    let aux = prox_simbolo();
    identifier();
    simbolo = prox_simbolo();

    if simbolo.tipo == Tokens::AbreColchete || simbolo.tipo == Tokens::Ponto {
        infipo();
        simbolo = prox_simbolo();
    }

    consome(simbolo, Tokens::Atribuicao);

    expr();
    fecha_expr();
    gera_mepa(["ARMZ ", parser::procura_var(aux.tok).to_string().as_ref()].join(""));
}

/*
infipo ::= [ expr { , expr } ] infipo
| . identifier infipo
| Vazio
*/
fn infipo() {
    let mut simbolo = prox_simbolo();
    
    match simbolo.tipo {
        Tokens::AbreColchete => {
            consome(simbolo, Tokens::AbreColchete);
            expr();
            simbolo = prox_simbolo();
            'infipo_loop: loop {
                if simbolo.tipo != Tokens::Virgula {
                    break 'infipo_loop;
                }

                consome(simbolo, Tokens::Virgula);
                expr();
                simbolo = prox_simbolo();
            }
            consome(simbolo, Tokens::FechaColchete);
        },
        Tokens::Ponto => {
            consome(simbolo, Tokens::Ponto);
            identifier();
            infipo();
        }
        _ => {}
    }
}

// expr_list ::= expr { , expr }
fn expr_list() {
    let mut simbolo;
    expr();
    simbolo = prox_simbolo();
    'expr_list_loop: loop {
        if simbolo.tipo != Tokens::Virgula {
            break 'expr_list_loop;
        }
        consome(simbolo, Tokens::Virgula);
        expr();
        simbolo = prox_simbolo();
    }
}

// expr ::= simple_expr [ relop simple_expr ]
fn expr() {
    let simbolo;
    simple_expr();
    simbolo = prox_simbolo();
    match simbolo.tipo {
        Tokens::Igual | Tokens::Menor | Tokens::Maior | Tokens::Diferente | Tokens::MaiorIgual | Tokens::MenorIgual | Tokens::In => {
            relop();
            simple_expr();
        },
        _ => {}
    }
}

fn fecha_expr(){
    let mut simbolo;

    loop {
        simbolo = desempilha();
        if simbolo != '('.to_string() && simbolo != "None".to_string() {
            if simbolo.parse::<i32>().is_ok() {
                gera_mepa(["CRCT", simbolo.as_ref()].join(" "));
            } else {
                gera_mepa(simbolo);
            }
        } else {
            break;
        }
    }
}

fn relop() {
    let simbolo = prox_simbolo();

    match simbolo.tipo {
        Tokens::Igual => consome(simbolo, Tokens::Igual),
        Tokens::Menor => consome(simbolo, Tokens::Menor),
        Tokens::Maior => consome(simbolo, Tokens::Maior),
        Tokens::Diferente => consome(simbolo, Tokens::Diferente),
        Tokens::MaiorIgual => consome(simbolo, Tokens::MaiorIgual),
        Tokens::MenorIgual => consome(simbolo, Tokens::MenorIgual),
        _ => consome(simbolo, Tokens::In),
    }
}

// simple_expr ::= [+|-] term { addop term }
fn simple_expr() {
    let mut simbolo;
    if (temSinal()){
        sinal();
    }

    term();
    simbolo = prox_simbolo();
    'simple_expr_loop: loop {
        if simbolo.tipo != Tokens::Mais && simbolo.tipo != Tokens::Menos && simbolo.tipo != Tokens::Or {
            break 'simple_expr_loop;
        }
        addop();
        term();
        simbolo = prox_simbolo();
    }
}

// addop ::= + | - | or
fn addop() {
    let simbolo = prox_simbolo();

    if simbolo.tipo == Tokens::Mais {
        let aux = desempilha();
        empilha(instrucao(Tokens::Mais));
        empilha(aux);
        consome(simbolo, Tokens::Mais);
    } else if simbolo.tipo == Tokens::Menos {
        let aux = desempilha();
        empilha(instrucao(Tokens::Menos));
        empilha(aux);
        consome(simbolo, Tokens::Menos);
    } else {
        consome(simbolo, Tokens::Or);
    }
}

// term ::= factor { mulop factor }
fn term() {
    let mut simbolo;
    factor();

    simbolo = prox_simbolo();
    'term_loop: loop {
        if simbolo.tipo != Tokens::Multiplicacao && simbolo.tipo != Tokens::Divisao && simbolo.tipo != Tokens::Div && simbolo.tipo != Tokens::Mod && simbolo.tipo != Tokens::And {
            break 'term_loop;
        }
        mulop();

        factor();
        simbolo = prox_simbolo();
    }
}

// mulop ::= * | / | div | mod | and
fn mulop() {
    let simbolo = prox_simbolo();

    if simbolo.tipo == Tokens::Multiplicacao {
        let aux = desempilha();
        empilha(instrucao(Tokens::Multiplicacao));
        consome(simbolo, Tokens::Multiplicacao);
    } else if simbolo.tipo == Tokens::Divisao {
        let aux = desempilha();
        empilha(instrucao(Tokens::Divisao));
        consome(simbolo, Tokens::Divisao);
    } else if simbolo.tipo == Tokens::Div {
        consome(simbolo, Tokens::Div);
    } else if simbolo.tipo == Tokens::Mod {
        consome(simbolo, Tokens::Mod);
    } else {
        consome(simbolo, Tokens::And);
    }
}

/*
factor ::= identifier infipo
| number
| string
| identifier [( expr_list )]
| ( expr )
| not factor
*/
fn factor() {
    let mut simbolo = prox_simbolo();
    
    if (ehString()){
        string();
    } else if simbolo.tipo == Tokens:: Numero {
        let aux = prox_simbolo();
        number();
        fecha_expr();
    } else if simbolo.tipo == Tokens::AbreParenteses {
        empilha('('.to_string());
        consome(simbolo, Tokens::AbreParenteses);
        expr();
        simbolo = prox_simbolo();
        fecha_expr();
        consome(simbolo, Tokens::FechaParenteses);
    } else if simbolo.tipo == Tokens::Not {
        consome(simbolo, Tokens::Not);
        factor();
    } else {
        if simbolo.tipo == Tokens::True {
            consome(simbolo, Tokens::True);
        } else if simbolo.tipo == Tokens::False  {
            consome(simbolo, Tokens::False);
        } else {
            let aux = prox_simbolo();
            let aux2 = desempilha();

            if (aux2.contains("CRVL")){
                empilha(["CRVL ",parser::procura_var(aux.tok).to_string().as_ref()].join(""));
                empilha(aux2);    
            } else {
                empilha(aux2);
                empilha(["CRVL ",parser::procura_var(aux.tok).to_string().as_ref()].join(""));
            }
            consome(simbolo, Tokens::Identificador);
        }
        simbolo = prox_simbolo();
        if simbolo.tipo == Tokens::AbreParenteses {
            empilha('('.to_string());
            consome(simbolo, Tokens::AbreParenteses);
            expr_list();
            simbolo = prox_simbolo();
            fecha_expr();
            consome(simbolo, Tokens::FechaParenteses);
        } else {
            infipo();
        }
    }
}
