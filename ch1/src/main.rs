mod error;
mod lexer;

use std::fs::File;

use error::Result;
use lexer::{Lexer, Token};

fn main() -> Result<()> {
    let file = File::open("tests/fib.kal")?;
    let mut lexer = Lexer::new(file);
    loop {
        let token = lexer.next_token()?;
        println!("{:?}", token);
        if token == Token::Eof {
            break;
        }
    }
    Ok(())
}
