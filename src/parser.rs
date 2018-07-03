use lexer;
use lexer::Tokens;
use std::sync::Mutex;

use std::process::exit;

#[derive(Debug, PartialEq, Clone)]
pub struct Simbolo {
    pub tok: String,
    pub tipo: Tokens,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variavel {
    pub tok: String,
    pub posicao: i32,
}

lazy_static! {
    pub static ref tabelaSimb: Mutex<Vec<Simbolo>> = Mutex::new(Vec::<Simbolo>::new());
    pub static ref tabelaVariaveis: Mutex<Vec<Variavel>> = Mutex::new(Vec::<Variavel>::new());
}

static mut linha: usize = 0;
static mut pos_atual: i32 = 0; // eh o contador das variaveis existentes

fn erro_msg(msg: String){
    println!("{}", msg);
    exit(1);
}

fn erro(token_lido: lexer::Token, token_esperado: Tokens){
    println!("ERRO na linha {0}: Esperado token {1}, mas encontrou {2}", token_lido.lin, token_esperado, token_lido.tipo);
    
    unsafe {
        print!("{:?}", lexer::tabelaToken.lock().unwrap()[linha]);
    }
    exit(1);
}

fn prox_token() -> lexer::Token {
    unsafe {
        if (!lexer::hasToken(linha)){
            linha = linha + 1;
        }
        return lexer::getToken(linha);
    }
}

fn consome(token_atual: lexer::Token, token_esperado: Tokens){
    if token_atual.tipo != token_esperado {
        erro(token_atual, token_esperado);
    }
    else {
        let simb = Simbolo {
            tok: token_atual.tok,
            tipo: token_atual.tipo,
        };

        tabelaSimb.lock().unwrap().push(simb);
        unsafe {
            lexer::eraseToken(linha);
        }
    }
}

pub fn ASD(){
    lexer::lexico();

    // let val = lexer::tabelaToken.lock().unwrap().len();

    // for i in 0..val {
    //     println!("linha {}\n", i+1);
    //     while (lexer::hasToken(i)){
    //         println!("{:?}", lexer::getToken(i));
    //     }
    // }

    program();
    print!("\n\nAnálise sintática não encontrou erros.");
}

// program ::= program identifier [ ( identifier_list ) ] ; block .
fn program(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Program);
    identifier();
    simbolo = prox_token();

    if (simbolo.tipo == Tokens::AbreParenteses){
        consome(simbolo, Tokens::AbreParenteses);
        identifier_list();
        simbolo = prox_token();
        consome(simbolo, Tokens::FechaParenteses);
        simbolo = prox_token();
    }

    consome(simbolo, Tokens::PontoVirgula);
    block();

    simbolo = prox_token();
    consome(simbolo, Tokens::Ponto);

    unsafe {
        let data = tabelaVariaveis.lock().unwrap();
        for i in 0..data.len(){
            println!("{:?}", data[i]);
        }
    }
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
    let mut simbolo = prox_token();
    if simbolo.tipo == Tokens::Label {
        label_declaration_part();
        simbolo = prox_token();
    }

    if simbolo.tipo == Tokens::Const{
        const_declaration_part();
        simbolo = prox_token();
    }

    if simbolo.tipo == Tokens::Type{
        type_declaration_part();
        simbolo = prox_token();
    }

    if simbolo.tipo == Tokens::Var{
        var_declaration_part();
        simbolo = prox_token();
    }

    if simbolo.tipo == Tokens::Procedure || simbolo.tipo == Tokens::Function {
         subroutine_declaration_part();
    }
    compound_statement();
}

// label_declaration_part ::= label number { , number } ;
fn label_declaration_part(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Label);
    number_list();
    simbolo = prox_token();
    consome(simbolo, Tokens::PontoVirgula);
}

// const_declaration_part ::= const const_definition { ; const_definition } ;
fn const_declaration_part(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Const);
    const_definition();
    simbolo = prox_token();

    unsafe {
        while simbolo.tipo == Tokens::PontoVirgula && lexer::lookahead_nextline(linha+1).tipo == Tokens::Identificador {
            consome(simbolo, Tokens::PontoVirgula);
            const_definition();
            simbolo = prox_token();
        }
    }

    consome(simbolo, Tokens::PontoVirgula);
}

// const_definition ::= identifier = const
fn const_definition(){
    identifier();
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Igual);
    const_();
}

// type_declaration_part ::= type type_definition { ; type_definition } ;
fn type_declaration_part(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Type);
    type_definition();
    'type_declaration_part_loop: loop {
        simbolo = prox_token();
        consome(simbolo, Tokens::PontoVirgula);
        simbolo = prox_token();
        if simbolo.tipo != Tokens::Identificador {
            break 'type_declaration_part_loop;
        }
        type_definition();
    }
}

// type_definition ::= identifier = type
fn type_definition() {
    let mut simbolo;
    identifier();
    simbolo = prox_token();
    consome(simbolo, Tokens::Igual);
    type_();
}

/*
type ::= ^ identifier
| array [ simple_type { , simple_type } ] of type
| set of simple_type
| record field_list end
| simple_type
*/
fn type_() {
    let simbolo = prox_token();

    match simbolo.tipo {
        Tokens::Boolean => consome(simbolo, Tokens::Boolean),
        Tokens::Char => consome(simbolo, Tokens::Char),
        Tokens::Integer => consome(simbolo, Tokens::Integer),
        Tokens::Real => consome(simbolo, Tokens::Real),
        Tokens::Identificador => identifier(),
        Tokens::Array => array(),
        Tokens::Set => set_of(),
        Tokens::Record => record(),
        _ => simple_type(),
    }
}

// array [ simple_type { , simple_type } ] of type
fn array() {
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Array);
    simbolo = prox_token();
    consome(simbolo, Tokens::AbreColchete);
    simple_type();
    simbolo = prox_token();
    while simbolo.tipo == Tokens::Virgula {
        consome(simbolo, Tokens::Virgula);
        simple_type();
        simbolo = prox_token();
    }
    simbolo = prox_token();
    consome(simbolo, Tokens::FechaColchete);
    simbolo = prox_token();
    consome(simbolo, Tokens::Of);
    type_();
}

// set of simple_type
fn set_of() {
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Set);
    simbolo = prox_token();
    consome(simbolo, Tokens::Of);
    simple_type();
}

// record field_list end
fn record() {
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Record);
    field_list();
    simbolo = prox_token();
    consome(simbolo, Tokens::End);
}

/*
simple_type ::= identifier
| ( identifier { , identifier } )
| const .. const
*/
fn simple_type(){
    let mut simbolo = prox_token();
    match simbolo.tipo {
        Tokens::Identificador => identifier(),
        Tokens::AbreParenteses => {
            consome(simbolo, Tokens::AbreParenteses);
            simbolo = prox_token();
            while simbolo.tipo == Tokens::Virgula {
                consome(simbolo, Tokens::Virgula);
                identifier();
                simbolo = prox_token();
            }
            consome(simbolo, Tokens::FechaParenteses);
        },
        _ => {
            const_();
            simbolo = prox_token();
            consome(simbolo, Tokens::Ponto);
            simbolo = prox_token();
            consome(simbolo, Tokens::Ponto);
            const_();
        }
    }
}

/*const ::= string
    | [+ | -] identifier
    | [+ | -] number
*/
fn const_() {
    let mut simbolo;
    // verifica se é string
    if (ehString()){
        string();
    } else {
        if (temSinal()){
            sinal();
        }
        simbolo = prox_token();
        if (simbolo.tipo == Tokens::Identificador){
            while (simbolo.tipo == Tokens::Identificador){
                identifier();
            }
        } else {
            number();
        }
    }
}

fn ehString() -> bool {
    let simbolo = prox_token();
    return simbolo.tipo == Tokens::Apostrofo || simbolo.tipo == Tokens::Aspas;
}

fn string(){
    // sinaliza se string abriu com aspas [true] ou apóstrofo [false]
    let mut tipo = false;
    let mut simbolo = prox_token();
    if simbolo.tipo == Tokens::Apostrofo {
        consome(simbolo, Tokens::Apostrofo);
    } else {
        tipo = true;
        consome(simbolo, Tokens::Aspas);
    }

    simbolo = prox_token();
    while (simbolo.tipo != Tokens::Apostrofo && simbolo.tipo != Tokens::Aspas){
        identifier();
        simbolo = prox_token();
    }
    if(!tipo){
        consome(simbolo, Tokens::Apostrofo);
    } else {
        consome(simbolo, Tokens::Aspas);
    }
}

fn ehSequenciaDigitos() -> bool {
    unsafe {
        let aux = lexer::lookahead(linha);
        return aux.tipo != Tokens::Ponto && aux.tipo != Tokens::FatorEscala;
    }
}

fn number_list(){
    number();

    let mut simbolo = prox_token();

    while simbolo.tipo == Tokens::Virgula {
        consome(simbolo, Tokens::Virgula);
        number();
        simbolo = prox_token();
    }
}

fn number() {
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Numero);

    simbolo = prox_token();
    unsafe {
        if simbolo.tipo == Tokens::Ponto && lexer::lookahead(linha).tipo == Tokens::Numero {
            consome(simbolo, Tokens::Ponto);
            simbolo = prox_token();
            consome(simbolo, Tokens::Numero);
            simbolo = prox_token();
        }
    }
}

fn temSinal() -> bool {
    let simbolo = prox_token();
    return simbolo.tipo == Tokens::Mais || simbolo.tipo == Tokens::Menos;
}

fn sinal(){
    let simbolo = prox_token();
    if simbolo.tipo == Tokens::Mais {
        consome(simbolo, Tokens::Mais)
    } else {
        consome(simbolo, Tokens::Menos);
    }
}

// field_list ::= [ identifier_list : type] { ; [identifier_list : type] }
fn field_list() {
    let mut simbolo = prox_token();

    match simbolo.tipo {
        Tokens::Identificador => {
            'field_list_loop: loop {
                if (simbolo.tipo == Tokens::End) {
                    break 'field_list_loop;
                }
                identifier_list();
                simbolo = prox_token();
                consome(simbolo, Tokens::DoisPontos);
                type_();
                simbolo = prox_token();
                if simbolo.tipo != Tokens::PontoVirgula {
                    break 'field_list_loop;
                }
                consome(simbolo, Tokens::PontoVirgula);
                simbolo = prox_token();
            }
        },
        _ => {},
    }
}

// var_declaration_part ::= var var_declaration { ; var_declaration} ;
fn var_declaration_part(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Var);
    var_declaration();

    simbolo = prox_token();
    unsafe {
        while simbolo.tipo == Tokens::PontoVirgula && lexer::lookahead_nextline(linha+1).tipo == Tokens::Identificador{
            consome(simbolo, Tokens::PontoVirgula);
            var_declaration();
            simbolo = prox_token();
        }
    }

    consome(simbolo, Tokens::PontoVirgula);
}

// var_declaration ::= identifier_list_var : type
fn var_declaration(){
    let mut simbolo;
    identifier_list_var();

    simbolo = prox_token();

    consome(simbolo, Tokens::DoisPontos);
    simbolo = prox_token();

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

// identifier_list_var ::= identifier { , identifier }
fn identifier_list_var(){
    let (mut simbolo, mut aux);
    aux = prox_token();
    identifier();
    unsafe {
        let mut var_atual = Variavel {
            tok: aux.tok,
            posicao: pos_atual,
        };
        tabelaVariaveis.lock().unwrap().push(var_atual); // adicionando variavel atual na tabelavariaveis
        pos_atual = pos_atual + 1;
    }
    simbolo = prox_token();
    while simbolo.tipo == Tokens::Virgula {
        consome(simbolo, Tokens::Virgula);
        aux = prox_token();
        identifier();
        unsafe {
            let mut var_atual = Variavel {
                tok: aux.tok,
                posicao: pos_atual,
            };
            tabelaVariaveis.lock().unwrap().push(var_atual); // adicionando variavel atual na tabelavariaveis
            pos_atual = pos_atual + 1;
        }
        simbolo = prox_token();
    }
}

// identifier_list ::= identifier { , identifier }
fn identifier_list(){
    let mut simbolo;

    identifier();
    simbolo = prox_token();
    while simbolo.tipo == Tokens::Virgula {
        consome(simbolo, Tokens::Virgula);
        identifier();
        simbolo = prox_token();
    }
}

fn identifier(){
    let simbolo = prox_token();
    consome(simbolo, Tokens::Identificador);
}

// subroutine_declaration_part ::= { procedure_declaration ; | function_declaration ; }
fn subroutine_declaration_part(){
    let mut simbolo = prox_token();

    'subroutine_declaration_part_loop: loop {
        match simbolo.tipo {
            Tokens::Procedure => procedure_declaration(),
            _ => function_declaration(),
        }

        simbolo = prox_token();
        consome(simbolo, Tokens::PontoVirgula);
        simbolo = prox_token();

        if (simbolo.tipo != Tokens::Procedure && simbolo.tipo != Tokens::Function){
            break 'subroutine_declaration_part_loop;
        }
    }
}

// procedure_declaration ::= procedure identifier [ formal_parameters ] ; block
fn procedure_declaration(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Procedure);
    identifier();

    formal_parameters();

    simbolo = prox_token();
    consome(simbolo, Tokens::PontoVirgula);
    block();
}

// function_declaration ::= function identifier [ formal_parameters ] : identifier ; block
fn function_declaration(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Function);
    identifier();

    formal_parameters();
    simbolo = prox_token();
    consome(simbolo, Tokens::DoisPontos);

    simbolo = prox_token();

    match simbolo.tipo {
        Tokens::Boolean => consome(simbolo, Tokens::Boolean),
        Tokens::Char => consome(simbolo, Tokens::Char),
        Tokens::Integer => consome(simbolo, Tokens::Integer),
        Tokens::Real => consome(simbolo, Tokens::Real),
        _ => identifier(),
    }

    simbolo = prox_token();
    consome(simbolo, Tokens::PontoVirgula);

    block();
}

// formal_parameters ::= (  param_section {  ;  param_section } )  
fn formal_parameters(){
    let mut simbolo = prox_token();
    if simbolo.tipo != Tokens::AbreParenteses {
        return;
    }

    consome(simbolo, Tokens::AbreParenteses);
    param_section();
    simbolo = prox_token();

    'formal_parameters_loop: loop {
        if simbolo.tipo != Tokens::PontoVirgula {
            break 'formal_parameters_loop;
        }

        consome(simbolo, Tokens::PontoVirgula);
        param_section();
        simbolo = prox_token();
    }

    consome(simbolo, Tokens::FechaParenteses);
}

/*
param_section ::= [ var ] identifier_list : identifier   
| function identifier_list : identifier   
| procedure identifier_list
*/
fn param_section(){
    let mut simbolo = prox_token();

    match simbolo.tipo {
        Tokens::Function => {
            consome(simbolo, Tokens::Function);
            identifier_list();
            simbolo = prox_token();
            consome(simbolo, Tokens::DoisPontos);
            simbolo = prox_token();
            match simbolo.tipo {
                Tokens::Boolean => consome(simbolo, Tokens::Boolean),
                Tokens::Char => consome(simbolo, Tokens::Char),
                Tokens::Integer => consome(simbolo, Tokens::Integer),
                Tokens::Real => consome(simbolo, Tokens::Real),
                _ => identifier(),
            }
        },
        Tokens::Procedure => {
            consome(simbolo, Tokens::Procedure);
            identifier_list();
        },
        _ => {
            if simbolo.tipo == Tokens::Var {
                consome(simbolo, Tokens::Var);
                simbolo = prox_token();
            }
            identifier_list();
            simbolo = prox_token();
            consome(simbolo, Tokens::DoisPontos);
            simbolo = prox_token();
            match simbolo.tipo {
                Tokens::Boolean => consome(simbolo, Tokens::Boolean),
                Tokens::Char => consome(simbolo, Tokens::Char),
                Tokens::Integer => consome(simbolo, Tokens::Integer),
                Tokens::Real => consome(simbolo, Tokens::Real),
                _ => identifier(),
            }
        }
    }
}

// compound_statement ::= begin labeled_statement { ; labeled_statement  } end
fn compound_statement() {
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Begin);

    labeled_statement();
    simbolo = prox_token();

    'compound_statement_loop: loop {
        if simbolo.tipo != Tokens::PontoVirgula {
            break 'compound_statement_loop;
        }

        consome(simbolo, Tokens::PontoVirgula);
        labeled_statement();
        simbolo = prox_token();
    }

    consome(simbolo, Tokens::End);
}

// labeled_statement ::= [ number :] statement   
fn labeled_statement() {
    let mut simbolo = prox_token();

    if simbolo.tipo == Tokens::Numero {
        if ehSequenciaDigitos() {
            consome(simbolo, Tokens::Numero);
            simbolo = prox_token();
            consome(simbolo, Tokens::DoisPontos);
        } else {
            let string = ["Erro: Esperado algum número inteiro mas foi encontrado um número não inteiro ({:?}).", simbolo.tok.as_ref()].join("\n");
            erro_msg(string);
        }
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
    let mut simbolo = prox_token();
    match simbolo.tipo {
        Tokens::Identificador => {
            unsafe {
                simbolo = lexer::lookahead(linha);
                match simbolo.tipo {
                    Tokens::AbreColchete | Tokens::Ponto | Tokens::Atribuicao => assign_statement(),
                    Tokens::AbreParenteses | _ => procedure_call(),
                }
            }
        },
        Tokens::If => if_statement(),
        Tokens::Case => case_statement(),
        Tokens::While => while_statement(),
        Tokens::Repeat => repeat_statement(),
        Tokens::For => for_statement(),
        Tokens::With => with_statement(),
        Tokens::Goto => goto_statement(),
        Tokens::Begin => compound_statement(),
        _ => internal_functions(),
    }
}

// assign_statement ::= identifier [ infipo ] := expr
fn assign_statement(){
    let mut simbolo;
    identifier();
    simbolo = prox_token();

    if simbolo.tipo == Tokens::AbreColchete || simbolo.tipo == Tokens::Ponto {
        infipo();
        simbolo = prox_token();
    }

    consome(simbolo, Tokens::Atribuicao);
    expr();
}

// procedure_call ::= identifier [ ( expr_list ) ]
fn procedure_call() {
    let mut simbolo;
    identifier();

    simbolo = prox_token();
    if simbolo.tipo == Tokens::AbreParenteses {
        consome(simbolo, Tokens::AbreParenteses);
        expr_list();
        simbolo = prox_token();
        consome(simbolo, Tokens::FechaParenteses);
    }
}

// if_statement ::= if expr then statement [ else statement ]
fn if_statement() {
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::If);
    expr();
    simbolo = prox_token();
    consome(simbolo, Tokens::Then);
    statement();
    simbolo = prox_token();
    if simbolo.tipo == Tokens::Else {
        consome(simbolo, Tokens::Else);
        statement();
    }
}

// while_statement ::= while expr do statement
fn while_statement(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::While);
    expr();
    simbolo = prox_token();
    consome(simbolo, Tokens::Do);
    statement();
}

// repeat_statement ::= repeat statement { ; statement } until expr
fn repeat_statement(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Repeat);
    statement();

    simbolo = prox_token();
    consome(simbolo, Tokens::Until);
    expr();
}

// for_statement ::= for identifier infipo := expr (to | downto) expr do statement
fn for_statement(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::For);
    identifier();
    infipo();
    simbolo = prox_token();
    consome(simbolo, Tokens::Atribuicao);
    simbolo = prox_token();
    match simbolo.tipo {
        Tokens::To => consome(simbolo, Tokens::To),
        Tokens::Downto => consome(simbolo, Tokens::Downto),
        _ => {
            let string = ["Erro: Esperado diretiva To ou Downto, mas foi encontrado", simbolo.tok.as_ref()].join("\n");
            erro_msg(string);
        }
    }

    expr();
    simbolo = prox_token();
    consome(simbolo, Tokens::Do);
    statement();
}

// with_statement ::= with identifier infipo { , identifier infipo } do statement
fn with_statement(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::With);
    identifier();
    infipo();
    simbolo = prox_token();
    'with_statement_loop: loop {
        if simbolo.tipo != Tokens::Virgula {
            break 'with_statement_loop;
        }
        consome(simbolo, Tokens::Virgula);
        identifier();
        infipo();
        simbolo = prox_token();
    }

    consome(simbolo, Tokens::Do);
    statement();
}

// case_statement ::= case expr of case { ; case } end
fn case_statement(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Case);
    expr();
    simbolo = prox_token();
    consome(simbolo, Tokens::Of);
    case();
    simbolo = prox_token();
    'case_statement_loop: loop {
        if simbolo.tipo != Tokens::PontoVirgula {
            break 'case_statement_loop;
        }

        consome(simbolo, Tokens::PontoVirgula);
        case();
        simbolo = prox_token();
    }

    consome(simbolo, Tokens::End);
}

// case ::= const {, const } : statement
fn case(){
    let mut simbolo;

    const_();
    simbolo = prox_token();
    'case_loop: loop {
        if (simbolo.tipo != Tokens::Virgula){
            break 'case_loop;
        }

        consome(simbolo, Tokens::Virgula);
        const_();
        simbolo = prox_token();
    }

    consome(simbolo, Tokens::DoisPontos);
    statement();
}

// goto_statement ::= goto number
fn goto_statement() {
    let simbolo = prox_token();
    consome(simbolo, Tokens::Goto);
    number();
}

/*
infipo ::= [ expr { , expr } ] infipo
| . identifier infipo
| Vazio
*/
fn infipo() {
    let mut simbolo = prox_token();
    
    match simbolo.tipo {
        Tokens::AbreColchete => {
            consome(simbolo, Tokens::AbreColchete);
            expr();
            simbolo = prox_token();
            'infipo_loop: loop {
                if simbolo.tipo != Tokens::Virgula {
                    break 'infipo_loop;
                }

                consome(simbolo, Tokens::Virgula);
                expr();
                simbolo = prox_token();
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
    simbolo = prox_token();
    'expr_list_loop: loop {
        if simbolo.tipo != Tokens::Virgula {
            break 'expr_list_loop;
        }
        consome(simbolo, Tokens::Virgula);
        expr();
        simbolo = prox_token();
    }
}

// expr ::= simple_expr [ relop simple_expr ]
fn expr() {
    let simbolo;
    simple_expr();
    simbolo = prox_token();
    match simbolo.tipo {
        Tokens::Igual | Tokens::Menor | Tokens::Maior | Tokens::Diferente | Tokens::MaiorIgual | Tokens::MenorIgual | Tokens::In => {
            relop();
            simple_expr();
        },
        _ => {}
    }
}

fn relop() {
    let simbolo = prox_token();

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
    simbolo = prox_token();
    'simple_expr_loop: loop {
        if simbolo.tipo != Tokens::Mais && simbolo.tipo != Tokens::Menos && simbolo.tipo != Tokens::Or {
            break 'simple_expr_loop;
        }
        addop();
        term();
        simbolo = prox_token();
    }
}

// addop ::= + | - | or
fn addop() {
    let simbolo = prox_token();

    if simbolo.tipo == Tokens::Mais {
        consome(simbolo, Tokens::Mais);
    } else if simbolo.tipo == Tokens::Menos {
        consome(simbolo, Tokens::Menos);
    } else {
        consome(simbolo, Tokens::Or);
    }
}

// term ::= factor { mulop factor }
fn term() {
    let mut simbolo;
    factor();

    simbolo = prox_token();
    'term_loop: loop {
        if simbolo.tipo != Tokens::Multiplicacao && simbolo.tipo != Tokens::Divisao && simbolo.tipo != Tokens::Div && simbolo.tipo != Tokens::Mod && simbolo.tipo != Tokens::And {
            break 'term_loop;
        }
        mulop();
        factor();
        simbolo = prox_token();
    }
}

// mulop ::= * | / | div | mod | and
fn mulop() {
    let simbolo = prox_token();

    if simbolo.tipo == Tokens::Multiplicacao {
        consome(simbolo, Tokens::Multiplicacao);
    } else if simbolo.tipo == Tokens::Divisao {
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
    let mut simbolo = prox_token();
    
    if (ehString()){
        string();
    } else if simbolo.tipo == Tokens:: Numero {
        number();
    } else if simbolo.tipo == Tokens::AbreParenteses {
        consome(simbolo, Tokens::AbreParenteses);
        expr();
        simbolo = prox_token();
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
            consome(simbolo, Tokens::Identificador);
        }
        simbolo = prox_token();
        if simbolo.tipo == Tokens::AbreParenteses {
            consome(simbolo, Tokens::AbreParenteses);
            expr_list();
            simbolo = prox_token();
            consome(simbolo, Tokens::FechaParenteses);
        } else {
            infipo();
        }
    }
}

/* internal_functions ::= read ( identifier_list )
| write ( identifier_list )
*/
fn internal_functions(){
    let mut simbolo = prox_token();

    match simbolo.tipo {
        Tokens::Read => {
            consome(simbolo, Tokens::Read);
            simbolo = prox_token();
            consome(simbolo, Tokens::AbreParenteses);
            identifier_list();
            simbolo = prox_token();
            consome(simbolo, Tokens::FechaParenteses);
        },
        Tokens::Write => {
            consome(simbolo, Tokens::Write);
            simbolo = prox_token();
            consome(simbolo, Tokens::AbreParenteses);
            identifier_list();
            simbolo = prox_token();
            consome(simbolo, Tokens::FechaParenteses);
        },
        _ => {},
    }
}
