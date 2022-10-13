use std::io::Error;

use super::token::Token;

pub struct Lexer {
    pos: usize,
    source: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            pos: 0,
            source,
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Result<&Vec<Token>, Error> {
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => self.read_space(),
                '(' | ')' | '{' | '}' | ':' | ';' | ',' | '.' | '+' | '-' | '*' | '/' | '%' => {
                    self.tokens.push(match ch {
                        '(' => Token::LeftParen,
                        ')' => Token::RightParen,
                        '{' => Token::LeftCurly,
                        '}' => Token::RightCurly,
                        ':' => Token::Colon,
                        ';' => Token::Semicolon,
                        ',' => Token::Comma,
                        '.' => Token::Dot,
                        '+' => Token::Add,
                        '-' => Token::Sub,
                        '*' => Token::Mul,
                        '/' => Token::Div,
                        '%' => Token::Mod,
                        _ => unreachable!(),
                    });
                    self.bump();
                }
                '0'..='9' => self.read_number(),
                'a'..='z' | 'A'..='Z' => self.read_word(),
                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Unexpected input",
                    ))
                }
            }
        }
        Ok(&self.tokens)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.pos)
    }

    fn bump(&mut self) {
        self.pos += 1;
    }

    fn read_space(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => self.bump(),
                _ => break,
            }
        }
    }

    fn read_number(&mut self) {
        let mut str = String::new();
        while let Some(ch) = self.peek() {
            match ch {
                '0'..='9' => {
                    str.push(ch);
                    self.bump();
                }
                _ => break,
            }
        }
        self.tokens.push(Token::Number(str.parse().unwrap()));
    }

    fn read_word(&mut self) {
        let mut str = String::new();
        while let Some(ch) = self.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' => {
                    str.push(ch);
                    self.bump();
                }
                _ => break,
            }
        }
        self.tokens.push(Token::Word(str));
    }
}
