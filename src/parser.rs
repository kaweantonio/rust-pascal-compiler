use lexer;
use lexer::Tokens;

use std::process::exit;

// #[derive(Debug, Clone)]
// pub enum Terminais {
//     NUMB, // Numeros
//     STRING, // Cadeia de caracteres
//     IDEN, // Identificador
//     COIDEN, // Identificador de constantes
//     FIIDEN, // Identificador de fields
//     VAIDEN, // Identificador de variáveis
//     FUIDEN, // Identificador de funções
//     TYIDEN, // Identificador de tipos
//     PRIDEN, // Identificador de procedimentos
// }

static mut linha: usize = 0;

fn erro_msg(msg: String){
    println!("{}", msg);
    exit(1);
}

fn erro(token_lido: lexer::Token, token_esperado: Tokens){
    println!("ERRO na linha {0}: Esperado token {1}, mas encontrou {2}", token_lido.lin, token_esperado, token_lido.tipo);
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
}

// program ::= program identifier [ ( identifier_list ) ] ; block .
fn program(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Program);
    identifier();
    simbolo = prox_token();

    consome(simbolo, Tokens::PontoVirgula);
    block();

    // simbolo = prox_token();
    // consome(simbolo, Tokens::Ponto);
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

    if simbolo.tipo == Tokens::Procedure{
         subroutine_declaration_part();
    }

    consome(simbolo, Tokens::Begin);
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
    while (simbolo.tipo == Tokens::Identificador){
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

// var_declaration ::= identifier_list : type
fn var_declaration(){
    let mut simbolo;
    identifier_list();

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
    let mut simbolo;
    simbolo = prox_token();

    match simbolo.tipo {
        Tokens::Procedure => procedure_declaration(),
        Tokens::Function => function_declaration(),
        _ => {
            let string = ["Erro: Esperado a declaracao de Procedure ou Function mas foi encontrado {:?}.", simbolo.tok.as_ref()].join("\n");
            erro_msg(string);
        },
    }

    simbolo = prox_token();
    consome(simbolo, Tokens::PontoVirgula);
}

// procedure_declaration ::= procedure identifier [ formal_parameters ] ; block
fn procedure_declaration(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Procedure);
    
    simbolo = prox_token();
    consome(simbolo, Tokens::AbreColchete);

    //parametros_formais();

    simbolo = prox_token();
    consome(simbolo, Tokens::FechaColchete);
    simbolo = prox_token();
    consome(simbolo, Tokens::PontoVirgula);
}

// function_declaration ::= function identifier [ formal_parameters ] : identifier ; block
fn function_declaration(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Function);
    simbolo = prox_token();
    consome(simbolo, Tokens::AbreColchete);

    //parametros_formais();

    simbolo = prox_token();
    consome(simbolo, Tokens::FechaColchete);
    simbolo = prox_token();
    consome(simbolo, Tokens::DoisPontos);

    identifier();

    simbolo = prox_token();
    consome(simbolo, Tokens::PontoVirgula);
}
