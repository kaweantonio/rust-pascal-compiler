#![feature(use_extern_macros)]

extern crate plex;

use std::io::Read;
use std::fs::File;
use std::io::prelude::*;

mod lexer {
    use plex::lexer;

    #[derive(Debug, Clone)]
    pub enum Token {
        // keywords
        Programa,
        Procedure,
        Function,
        Div,
        Or,
        And,
        Not,
        If,
        Then,
        Else,
        Of,
        While,
        Do,
        Begin,
        End,
        Read,
        Write,
        Var,
        Array,

        // Tipos
        True,
        False,
        Char(char),
        Stri(String),
        Ident(String),
        Inteiro(i64),

        // Simbolos
        Mais, // +
        Menos, // -
        Asterisco, // *
        BarraDir, // /
        Igual, // =
        Diferente, // <>
        Menor, // <
        Maior, // >
        MenorIgual, // <=
        MaiorIgual, // >=
        ParenEsq, // (
        ParenDir, // )
        ColchEsq, // [
        ColchDir, // ]
        Atribuicao, // :=
        Ponto, // .
        Virgula, // ,
        PontoVirgula, // ;
        DoisPontos, // :
        PontoPonto, // ..
        Apostrofo, // '
        Aspas, // "


        Whitespace,
        Comment,
    }

    lexer! {
        fn next_token(text: 'a) -> (Token, &'a str);

        r#"[ \t\r\n]+"# => (Token::Whitespace, text),
        // "C-style" comments (/* .. */) - can't contain "*/"
        r#"/[*](~(.*[*]/.*))[*]/"# => (Token::Comment, text),
        // "C++-style" comments (// ...)
        r#"//[^\n]*"# => (Token::Comment, text),

        //r#"print"# => (Token::Print, text),

        r#"[1-9][0-9]*"# => {
            (if let Ok(i) = text.parse() {
                Token::Inteiro(i)
            } else {
                panic!("integer {} is out of range", text)
            }, text)
        }

        //Palavras Reservadas
        r#"[pP][rR][oO][gG][rR][aA][mM]"# => (Token::Programa, text),
        r#"[pP][rR][oO][cC][eE][dD][uU][rR][eE]"# => (Token::Procedure,text),
        r#"[fF][uU][nN][cC][tT][iI][oO][nN]"# => (Token::Function, text),
        r#"[dD][iI][vV]"# => (Token::Div, text),
        r#"[oO][rR]"# => (Token::Or, text),
        r#"[aA][nN][dD]"# => (Token::And, text),
        r#"[nN][oO][tT]"# => (Token::Not, text),
        r#"[iI][fF]"# => (Token::If, text),
        r#"[tT][hH][eE][nN]"# => (Token::Then, text),
        r#"[eE][lL][sS][eE]"# => (Token::Else, text),
        r#"[oO][fF]"# => (Token::Of, text),
        r#"[wW][hH][iI][lL][eE]"# => (Token::While, text),
        r#"[dD][oO]"# => (Token::Do, text),
        r#"[bB][eE][gG][iI][nN]"# => (Token::Begin, text),
        r#"[eE][nN][dD]"# => (Token::End, text),
        r#"[rR][eE][aA][dD]"# => (Token::Read, text),
        r#"[wW][rR][iI][tT][eE]"# => (Token::Write, text),
        r#"[vV][aA][rR]"# => (Token::Var, text),
        r#"[aA][rR][rR][aA][yY]"# => (Token::Array, text),
        r#"[tT][rR][uU][eE]"# => (Token::True, text),
        r#"[fF][aA][lL][sS][eE]"# => (Token::False, text),

        r#"[a-zA-Z_][a-zA-Z0-9_]*"# => (Token::Ident(text.to_owned()), text),

        //Operadores
        r#"\+"# => (Token::Mais, text),
        r#"-"# => (Token::Menos, text),
        r#"\*"# => (Token::Asterisco, text),
        r#"/"# => (Token::BarraDir, text),

        //Operadores Lógicos
        r#"="# => (Token::Igual, text),
        r#"<>"# => (Token::Diferente, text),
        r#"<"# => (Token::Menor, text),
        r#">"# => (Token::Maior, text),
        r#"<="# => (Token::MenorIgual, text),
        r#">="# => (Token::MaiorIgual, text),

        // Outros Simbolos
        r#"\("# => (Token::ParenEsq, text),
        r#"\)"# => (Token::ParenDir, text),
        r#"\["# => (Token::ColchEsq, text),
        r#"\]"# => (Token::ColchDir, text),
        r#":="# => (Token::Atribuicao, text),
        r#"\.\."# => (Token::PontoPonto, text),
        r#"\."# => (Token::Ponto, text),
        r#","# => (Token::Virgula, text),
        r#";"# => (Token::PontoVirgula, text),
        r#":"# => (Token::DoisPontos, text),
        r#"\'"# => (Token::Apostrofo, text),
        r#"\""# => (Token::Aspas, text),

        r#"."# => panic!("unexpected character: {}", text),
    }

    pub struct Lexer<'a> {
        original: &'a str,
        remaining: &'a str,
    }

    impl<'a> Lexer<'a> {
        pub fn new(s: &'a str) -> Lexer<'a> {
            Lexer { original: s, remaining: s }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Span {
        pub lo: usize,
        pub hi: usize,
    }

    fn span_in(s: &str, t: &str) -> Span {
        let lo = s.as_ptr() as usize - t.as_ptr() as usize;
        Span {
            lo: lo,
            hi: lo + s.len(),
        }
    }

    impl<'a> Iterator for Lexer<'a> {
        type Item = (Token, Span);
        fn next(&mut self) -> Option<(Token, Span)> {
            loop {
                let tok = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                    self.remaining = new_remaining;
                    tok
                } else {
                    return None
                };
                match tok {
                    (Token::Whitespace, _) | (Token::Comment, _) => {
                        continue;
                    }
                    (tok, span) => {
                        return Some((tok, span_in(span, self.original)));
                    }
                }
            }
        }
    }
}

mod ast {
    use lexer::Span;

    #[derive(Debug)]
    pub struct Program {
        pub stmts: Vec<Expr>
    }

    #[derive(Debug)]
    pub struct Expr {
        pub span: Span,
        pub node: Expr_,
    }

    #[derive(Debug)]
    pub enum Expr_ {
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
        Mul(Box<Expr>, Box<Expr>),
        Div(Box<Expr>, Box<Expr>),
        Var(String),
        Assign(String, Box<Expr>),
        Print(Box<Expr>),
        Literal(i64),
    }
}

mod parser {
    use ast::*;
    use lexer::Token::*;
    use lexer::*;
    use plex::parser;
    parser! {
        fn parse_(Token, Span);

        // combine two spans
        (a, b) {
            Span {
                lo: a.lo,
                hi: b.hi,
            }
        }

        program: Program {
            statements[s] => Program { stmts: s }
        }

        statements: Vec<Expr> {
            => vec![],
            statements[mut st] atom[e] => {
                st.push(e);
                st
            }
        }
/*
        assign: Expr {
            Print assign[a] => Expr {
                span: span!(),
                node: Expr_::Print(Box::new(a)),
            },
            Ident(var) Equals assign[rhs] => Expr {
                span: span!(),
                node: Expr_::Assign(var, Box::new(rhs)),
            },
            term[t] => t,
        }

        term: Expr {
            term[lhs] Plus fact[rhs] => Expr {
                span: span!(),
                node: Expr_::Add(Box::new(lhs), Box::new(rhs)),
            },
            term[lhs] Minus fact[rhs] => Expr {
                span: span!(),
                node: Expr_::Sub(Box::new(lhs), Box::new(rhs)),
            },
            fact[x] => x
        }

        fact: Expr {
            fact[lhs] Star atom[rhs] => Expr {
                span: span!(),
                node: Expr_::Mul(Box::new(lhs), Box::new(rhs)),
            },
            fact[lhs] Slash atom[rhs] => Expr {
                span: span!(),
                node: Expr_::Div(Box::new(lhs), Box::new(rhs)),
            },
            atom[x] => x
        }
*/
        atom: Expr {
            // round brackets to destructure tokens
            ParenEsq => Expr {
                span: span!(),
                node: Expr_::Var(r#"\)"#.to_string()),
            },
            ParenDir => Expr {
                span: span!(),
                node: Expr_::Var(r#"\("#.to_string()),
            },
            ColchEsq => Expr {
                span: span!(),
                node: Expr_::Var(r#">="#.to_string()),
            },
            ColchDir => Expr {
                span: span!(),
                node: Expr_::Var(r#">="#.to_string()),
            },
            Atribuicao => Expr {
                span: span!(),
                node: Expr_::Var(r#">="#.to_string()),
            },
            PontoPonto => Expr {
                span: span!(),
                node: Expr_::Var(r#".."#.to_string()),
            },
            Ponto => Expr {
                span: span!(),
                node: Expr_::Var(r#"."#.to_string()),
            },
            Virgula => Expr {
                span: span!(),
                node: Expr_::Var(r#","#.to_string()),
            },
            PontoVirgula => Expr {
                span: span!(),
                node: Expr_::Var(r#";"#.to_string()),
            },
            DoisPontos => Expr {
                span: span!(),
                node: Expr_::Var(r#":"#.to_string()),
            },
            Apostrofo => Expr {
                span: span!(),
                node: Expr_::Var(r#"\'"#.to_string()),
            },
            Aspas => Expr {
                span: span!(),
                node: Expr_::Var(r#"\""#.to_string()),
            },
            Menos => Expr {
                span: span!(),
                node: Expr_::Var(r#"-"#.to_string()),
            },
            Igual => Expr {
                span: span!(),
                node: Expr_::Var(r#"="#.to_string()),
            },
            Asterisco => Expr {
                span: span!(),
                node: Expr_::Var(r#"\*"#.to_string()),
            },
            BarraDir => Expr {
                span: span!(),
                node: Expr_::Var(r#"/"#.to_string()),
            },
            Mais => Expr {
                span: span!(),
                node: Expr_::Var(r#"\+"#.to_string()),
            },
            Menor => Expr {
                span: span!(),
                node: Expr_::Var(r#"<"#.to_string()),
            },
            Maior => Expr {
                span: span!(),
                node: Expr_::Var(r#">"#.to_string()),
            },
            MenorIgual => Expr {
                span: span!(),
                node: Expr_::Var(r#"<="#.to_string()),
            },
            MaiorIgual => Expr {
                span: span!(),
                node: Expr_::Var(r#">="#.to_string()),
            },
            Diferente => Expr {
                span: span!(),
                node: Expr_::Var(r#"<>"#.to_string()),
            },
            Ident(i) => Expr {
                span: span!(),
                node: Expr_::Var(i),
            },
            Inteiro(i) => Expr {
                span: span!(),
                node: Expr_::Literal(i),
            },
            Programa => Expr {
                span: span!(),
                node: Expr_::Var(r#"[pP][rR][oO][gG][rR][aA][mM]"#.to_string()),
            },
            Procedure => Expr {
                span: span!(),
                node: Expr_::Var(r#"[pP][rR][oO][cC][eE][dD][uU][rR][eE]"#.to_string()),
            },
            Function => Expr {
                span: span!(),
                node: Expr_::Var(r#"[fF][uU][nN][cC][tT][iI][oO][nN]"#.to_string()),
            },
            Div => Expr {
                span: span!(),
                node: Expr_::Var(r#"[dD][iI][vV]"#.to_string()),
            },
            Or => Expr {
                span: span!(),
                node: Expr_::Var(r#"[oO][rR]"#.to_string()),
            },
            And => Expr {
                span: span!(),
                node: Expr_::Var(r#"[aA][nN][dD]"#.to_string()),
            },
            Not => Expr {
                span: span!(),
                node: Expr_::Var(r#"[nN][oO][tT]"#.to_string()),
            },
            If => Expr {
                span: span!(),
                node: Expr_::Var(r#"[iI][fF]"#.to_string()),
            },
            Then => Expr {
                span: span!(),
                node: Expr_::Var(r#"[tT][hH][eE][nN]"#.to_string()),
            },
            Else => Expr {
                span: span!(),
                node: Expr_::Var(r#"[eE][lL][sS][eE]"#.to_string()),
            },
            Of => Expr {
                span: span!(),
                node: Expr_::Var(r#"[oO][fF]"#.to_string()),
            },
            While => Expr {
                span: span!(),
                node: Expr_::Var(r#"[wW][hH][iI][lL][eE]"#.to_string()),
            },
            Do => Expr {
                span: span!(),
                node: Expr_::Var(r#"[wW][hH][iI][lL][eE]"#.to_string()),
            },
            Begin => Expr {
                span: span!(),
                node: Expr_::Var(r#"[bB][eE][gG][iI][nN]"#.to_string()),
            },
            End => Expr {
                span: span!(),
                node: Expr_::Var(r#"[eE][nN][dD]"#.to_string()),
            },
            Read => Expr {
                span: span!(),
                node: Expr_::Var(r#"[rR][eE][aA][dD]"#.to_string()),
            },
            Write => Expr {
                span: span!(),
                node: Expr_::Var(r#"[wW][rR][iI][tT][eE]"#.to_string()),
            },
            Var => Expr {
                span: span!(),
                node: Expr_::Var(r#"[vV][aA][rR]"#.to_string()),
            },
            Array => Expr {
                span: span!(),
                node: Expr_::Var(r#"[aA][rR][rR][aA][yY]"#.to_string()),
            },
            True => Expr {
                span: span!(),
                node: Expr_::Var(r#"[tT][rR][uU][eE]"#.to_string()),
            },
            False => Expr {
                span: span!(),
                node: Expr_::Var(r#"[fF][aA][lL][sS][eE]"#.to_string()),
            },
            //LParen assign[a] RParen => a
        }
    }

    pub fn parse<I: Iterator<Item=(Token, Span)>>(i: I) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
        parse_(i)
    }
}

mod interp {
    use ast::*;
    use std::collections::HashMap;

    pub fn interp<'a>(p: &'a Program) {
        let mut env = HashMap::new();
        for expr in &p.stmts {
            interp_expr(&mut env, expr);
        }
    }
    fn interp_expr<'a>(env: &mut HashMap<&'a str, i64>, expr: &'a Expr) -> i64 {
        use ast::Expr_::*;
         match expr.node {
            Add(ref a, ref b) => interp_expr(env, a) + interp_expr(env, b),
            Sub(ref a, ref b) => interp_expr(env, a) - interp_expr(env, b),
            Mul(ref a, ref b) => interp_expr(env, a) * interp_expr(env, b),
            Div(ref a, ref b) => interp_expr(env, a) / interp_expr(env, b),
            Assign(ref var, ref b) => {
                let val = interp_expr(env, b);
                env.insert(var, val);
                val
            }
            Var(ref var) => *env.get(&var[..]).unwrap(),
            Literal(lit) => lit,
            Print(ref e) => {
                let val = interp_expr(env, e);
                println!("{}", val);
                val
            }
        }
    }
}

fn main() {
    let mut s = String::new();

    let mut f = File::open("src/input.txt").expect("Unable to open file");
    f.read_to_string(&mut s).unwrap();
    println!("Arquivo Input:\n");
    println!("{}", s);

    //std::io::stdin().read_to_string(&mut s).unwrap();
    let lexer = lexer::Lexer::new(&s)
        .inspect(|tok| println!("tok: {:?}", tok));

    let program = parser::parse(lexer).unwrap();
    //interp::interp(&program);
}
