use crate::lexer:: { Lexer, Token };

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current_token: None,
        }
    }

    pub fn parse(&mut self) {

    }
}