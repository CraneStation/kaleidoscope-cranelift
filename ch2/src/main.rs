mod ast;
mod error;
mod lexer;
mod parser;

use std::io::{Write, stdin, stdout};

use error::Result;
use lexer::{Lexer, Token};
use parser::Parser;

fn main() -> Result<()> {
    let stdin = stdin();
    let lexer = Lexer::new(stdin);
    let mut parser = Parser::new(lexer);
    print!("ready> ");
    stdout().flush()?;
    loop {
        let token =
            match parser.lexer.peek() {
                Ok(ref token) => *token,
                Err(error) => {
                    eprintln!("Error: {:?}", error);
                    continue;
                },
            };
        match token {
            Token::Eof => break,
            Token::SemiColon => {
                parser.lexer.next_token()?;
                continue;
            },
            Token::Def => {
                match parser.definition() {
                    Ok(definition) => println!("{:?}", definition),
                    Err(error) => {
                        parser.lexer.next_token()?;
                        eprintln!("Error: {:?}", error);
                    },
                }
            },
            Token::Extern => {
                match parser.extern_() {
                    Ok(prototype) => println!("{:?}", prototype),
                    Err(error) => {
                        parser.lexer.next_token()?;
                        eprintln!("Error: {:?}", error);
                    },
                }
            },
            _ => {
                match parser.toplevel() {
                    Ok(expr) => println!("{:?}", expr),
                    Err(error) => {
                        parser.lexer.next_token()?;
                        eprintln!("Error: {:?}", error);
                    },
                }
            },
        }
        print!("ready> ");
        stdout().flush()?;
    }
    Ok(())
}
