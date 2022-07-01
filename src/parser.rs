use crate::lexer:: { Token };

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current_token: &'a Token,
    position: usize,
}

impl <'a> Parser <'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens,
            position: 0,
            current_token: &tokens[0]
        }
    }

    pub fn parse(&mut self) {
        loop {
            match self.current_token {
                Token::EOF => return,
                _ => {
                    println!("{:?}", self.current_token);
                    self.next();
                }
            }
        }
    }

    pub fn next(&mut self) {
        if let Token::EOF = self.current_token {
            return;
        }

        if self.position >= self.tokens.len() {
            return;
        }

        self.position += 1;
        self.current_token = &self.tokens[self.position];
    }
}