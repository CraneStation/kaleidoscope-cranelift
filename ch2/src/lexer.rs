use std::io::{
    Bytes,
    Read,
};
use std::iter::Peekable;

use crate::error::Result;
use crate::error::Error::UnknownChar;

#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,

    // Commands.
    Def,
    Extern,

    // Primary.
    Identifier(String),
    Number(f64),

    // Operators.
    LessThan,
    Minus,
    Plus,
    Star,

    // Other.
    SemiColon,
    OpenParen,
    CloseParen,
    Comma,
}

pub struct Lexer<R: Read> {
    bytes: Peekable<Bytes<R>>,
    lookahead: Option<Token>,
}

impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            bytes: reader.bytes().peekable(),
            lookahead: None,
        }
    }

    fn comment(&mut self) -> Result<Token> {
        loop {
            if let Some(char) = self.peek_char()? {
                self.bytes.next();
                if char == '\n' {
                    break;
                }
            }
            else {
                return Ok(Token::Eof);
            }
        }
        self.next_token()
    }

    fn digits(&mut self) -> Result<String> {
        let mut buffer = String::new();
        loop {
            if let Some(char) = self.peek_char()? {
                if char.is_numeric() {
                    self.bytes.next();
                    buffer.push(char);
                    continue;
                }
            }
            break;
        }

        Ok(buffer)
    }

    fn identifier(&mut self) -> Result<Token> {
        let mut ident = String::new();
        loop {
            if let Some(char) = self.peek_char()? {
                if char.is_ascii_alphanumeric() {
                    self.bytes.next();
                    ident.push(char);
                    continue;
                }
            }
            break;
        }
        let token =
            match ident.as_str() {
                "def" => Token::Def,
                "extern" => Token::Extern,
                _ => Token::Identifier(ident),
            };
        Ok(token)
    }

    pub fn next_token(&mut self) -> Result<Token> {
        if let Some(lookahead) = self.lookahead.take() {
            return Ok(lookahead);
        }
        if let Some(&Ok(byte)) = self.bytes.peek() {
            return match byte {
                b' ' | b'\n' | b'\r' | b'\t' => {
                    self.bytes.next();
                    self.next_token()
                },
                b'a' ..= b'z' | b'A' ..= b'Z' => self.identifier(),
                b'0' ..= b'9' | b'.' => self.number(),
                b'#' => self.comment(),
                _ => {
                    self.bytes.next();
                    let token =
                        match byte {
                            b'<' => Token::LessThan,
                            b'+' => Token::Plus,
                            b'-' => Token::Minus,
                            b'*' => Token::Star,
                            b';' => Token::SemiColon,
                            b',' => Token::Comma,
                            b'(' => Token::OpenParen,
                            b')' => Token::CloseParen,
                            _ => return Err(UnknownChar(byte as char)),
                        };
                    Ok(token)
                },
            }
        }

        match self.bytes.next() {
            Some(Ok(_)) => unreachable!(),
            Some(Err(error)) => Err(error.into()),
            None => Ok(Token::Eof),
        }
    }

    fn number(&mut self) -> Result<Token> {
        let integral = self.digits()?;
        if let Some('.') = self.peek_char()? {
            self.bytes.next();
            let decimals = self.digits()?;
             Ok(Token::Number(format!("{}.{}", integral, decimals).parse()?))
        }
        else {
            Ok(Token::Number(integral.parse()?))
        }
    }

    pub fn peek(&mut self) -> Result<&Token> {
        match self.lookahead {
            Some(ref token) => Ok(token),
            None => {
                self.lookahead = Some(self.next_token()?);
                self.peek()
            },
        }
    }

    fn peek_char(&mut self) -> Result<Option<char>> {
        if let Some(&Ok(byte)) = self.bytes.peek() {
            return Ok(Some(byte as char));
        }

        match self.bytes.next() {
            Some(Ok(_)) => unreachable!(),
            Some(Err(error)) => Err(error.into()),
            None => Ok(None),
        }
    }
}
