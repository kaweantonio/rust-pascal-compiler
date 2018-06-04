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
//     VAIDEN, // Identificador de variaveis
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

    programa();
}

// program ::= program identifier [ ( identifier_list ) ] ; block .
fn programa(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Program);
    identificador();
    simbolo = prox_token();

    consome(simbolo, Tokens::PontoVirgula);
    bloco();

    // simbolo = prox_token();
    // consome(simbolo, Tokens::Ponto);
}

// identifier_list ::= identifier { , identifier }
fn lista_de_identificadores(){
    let mut simbolo;

    identificador();
    simbolo = prox_token();
    while simbolo.tipo == Tokens::Virgula {
        consome(simbolo, Tokens::Virgula);
        identificador();
        simbolo = prox_token();
    }
}

fn identificador(){
    let simbolo = prox_token();
    consome(simbolo, Tokens::Identificador);
}

fn ehSequenciaDigitos() -> bool {
    unsafe {
        let aux = lexer::lookahead(linha);
        return aux.tipo != Tokens::Ponto && aux.tipo != Tokens::FatorEscala;
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
fn bloco(){
    let mut simbolo = prox_token();
    if simbolo.tipo == Tokens::Label {
        rotulos();
        simbolo = prox_token();
    }

    if simbolo.tipo == Tokens::Const{
        constantes();
        simbolo = prox_token();
    }

    if simbolo.tipo == Tokens::Type{
        tipo_declaracao();
        simbolo = prox_token();
    }

    if simbolo.tipo == Tokens::Var{
        variaveis();
        simbolo = prox_token();
    }

    // if simbolo.tipo == Tokens::Procedure{
    //     rotinas();
    // }

    consome(simbolo, Tokens::Begin);
}

// label_declaration_part ::= label number { , number } ;
fn rotulos(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Label);
    lista_de_numeros();
    simbolo = prox_token();
    consome(simbolo, Tokens::PontoVirgula);
}

fn lista_de_numeros(){
    numeros();

    let mut simbolo = prox_token();

    while simbolo.tipo == Tokens::Virgula {
        consome(simbolo, Tokens::Virgula);
        numeros();
        simbolo = prox_token();
    }
}

fn numeros() {
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

// const_declaration_part ::= const const_definition { ; const_definition } ;
fn constantes(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Const);
    const_definicao();
    simbolo = prox_token();
    
    unsafe {
        while simbolo.tipo == Tokens::PontoVirgula && lexer::lookahead_nextline(linha+1).tipo == Tokens::Identificador {
            consome(simbolo, Tokens::PontoVirgula);
            const_definicao();
            simbolo = prox_token();
        }
    }

    consome(simbolo, Tokens::PontoVirgula);
}

// const_definition ::= identifier = const
fn const_definicao(){
    identificador();
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Igual);
    const_();
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
                identificador();
            }
        } else {
            numeros();
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
        identificador();
        simbolo = prox_token();
    }
    if(!tipo){
        consome(simbolo, Tokens::Apostrofo);
    } else {
        consome(simbolo, Tokens::Aspas);
    }
}

// type_declaration_part ::= type type_definition { ; type_definition } ;
fn tipo_declaracao(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Type);
    tipo_definicao();
    'type_declaration_part_loop: loop {
        simbolo = prox_token();
        consome(simbolo, Tokens::PontoVirgula);
        simbolo = prox_token();
        if simbolo.tipo != Tokens::Identificador {
            break 'type_declaration_part_loop;
        }
        tipo_definicao();
    }
}

// type_definition ::= identifier = type
fn tipo_definicao() {
    let mut simbolo;
    identificador();
    simbolo = prox_token();
    consome(simbolo, Tokens::Igual);
    tipo();
}

/*
type ::= ^ identifier
| array [ simple_type { , simple_type } ] of type
| set of simple_type
| record field_list end
| simple_type
*/
fn tipo() {
    let simbolo = prox_token();

    match simbolo.tipo {
        Tokens::Boolean => consome(simbolo, Tokens::Boolean),
        Tokens::Char => consome(simbolo, Tokens::Char),
        Tokens::Integer => consome(simbolo, Tokens::Integer),
        Tokens::Real => consome(simbolo, Tokens::Real),
        Tokens::Identificador => identificador(),
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
    tipo();
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
        Tokens::Identificador => identificador(),
        Tokens::AbreParenteses => {
            consome(simbolo, Tokens::AbreParenteses);
            simbolo = prox_token();
            while simbolo.tipo == Tokens::Virgula {
                consome(simbolo, Tokens::Virgula);
                identificador();
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

// field_list ::= [ identifier_list : type] { ; [identifier_list : type] }
fn field_list() {
    let mut simbolo = prox_token();

    match simbolo.tipo {
        Tokens::Identificador => {
            'field_list_loop: loop {
                if (simbolo.tipo == Tokens::End) {
                    break 'field_list_loop;
                }
                lista_de_identificadores();
                simbolo = prox_token();
                consome(simbolo, Tokens::DoisPontos);
                tipo();
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
fn variaveis(){
    let mut simbolo = prox_token();
    consome(simbolo, Tokens::Var);
    var_declaracao();

    simbolo = prox_token();
    unsafe {
        while simbolo.tipo == Tokens::PontoVirgula && lexer::lookahead_nextline(linha+1).tipo == Tokens::Identificador{
            consome(simbolo, Tokens::PontoVirgula);
            var_declaracao();
            simbolo = prox_token();
        }
    }

    consome(simbolo, Tokens::PontoVirgula);
}

// var_declaration ::= identifier_list : type
fn var_declaracao(){
    let mut simbolo;
    lista_de_identificadores();

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
