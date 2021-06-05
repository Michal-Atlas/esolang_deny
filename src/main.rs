use logos::Logos;
use std::io::{stdin, Read};
use tokens::Token;
use variables::VariableContainer;

mod tokens;
mod variables;

fn main() {
    let mut vc = VariableContainer::new();
    let mut buff = String::new();
    stdin().read_to_string(&mut buff).unwrap();
    let mut lex = Token::lexer(&buff);

    loop {
        match lex.next() {
            Some(Token::Print) => {
                while lex.next() == Some(Token::Variable) {
                    print!("{}", vc.get(lex.slice()))
                }
            }
            Some(Token::Printf) => {
                while lex.next() == Some(Token::Variable) {
                    print!("{}", vc.get(lex.slice()) as char)
                }
            }
            Some(Token::Variable) => {
                let var = lex.slice();
                match lex.next() {
                    Some(Token::Assign) => {
                        lex.next();
                        vc.set(var, lex.slice().parse().unwrap());
                    }
                    Some(Token::Intake) => {
                        lex.next();
                        let mut a = [0u8; 1];
                        stdin().read(&mut a).unwrap();
                        vc.set(var, a[0]);
                    }
                    Some(s) => {
                        lex.next();
                        let a = vc.get(lex.slice());
                        lex.next();
                        let b = vc.get(lex.slice());
                        vc.set(
                            var,
                            match s {
                                Token::Sum => a + b,
                                Token::Diff => a - b,
                                Token::Product => a * b,
                                Token::Div => a / b,
                                Token::Mod => a % b,
                                _ => 0,
                            },
                        );
                    }
                    /*Some(Token::Subtract) => {
                        lex.next();
                        let a = vc.get(lex.slice());
                        lex.next();
                        let b = vc.get(lex.slice());
                        vc.set(var, a - b);
                    }*/
                    _ => panic!("{} [{:?}]", lex.slice(), lex.span()),
                };
            }
            None => break,
            _ => {}
        }
    }
}
