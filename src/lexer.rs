#[derive(Debug)]
pub enum Token {
    Number(u32),
    Str(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    LeftPar,
    RightPar,
}

pub struct Lexer {
    input: String,
    position: usize,
    current_character: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            current_character: input.chars().next(),
            input,
            position: 0,
        }
    }

    fn ok_next(&mut self, token: Token) -> Result<Token, String> {
        self.next();
        Ok(token)
    }

    pub fn get_tokens(&mut self) -> Result<std::vec::Vec<Token>, String> {
        let mut tokens = vec![];

        while let Some(t) = self.current_character {
            let token = match t {
                ' ' | '\t' | '\r' | '\n' => {self.next(); continue;}
                '0'..='9' => self.read_number(),
                '"' => self.read_string(),
                '+' => self.ok_next(Token::Plus),
                '-' => self.ok_next(Token::Minus),
                '*' => self.ok_next(Token::Multiply),
                '/' => self.ok_next(Token::Divide),
                '=' => self.ok_next(Token::Equals),
                '(' => self.ok_next(Token::LeftPar),
                ')' => self.ok_next(Token::RightPar),
                _ => Err(String::from(format!("Invalid character {}", self.current_character.unwrap()))),
            }?;
            
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next(&mut self) {
        self.position += 1;
        self.current_character = self.input.chars().nth(self.position);
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut final_number = 0;

        loop {
            let current_digit = match self.current_character {
                None => break,
                Some(c) => if let Some(d) = c.to_digit(10) { d } else { break },
            };
                        
            final_number = final_number * 10 + current_digit as u32;
            self.next()
        }

        Ok(Token::Number(final_number))
    }

    fn read_string(&mut self) -> Result<Token, String> {
        let mut full_string = String::new();
        
        let quote_symbol = self.current_character.unwrap();
        self.next();

        while let Some(chr) = self.current_character {
            if chr == quote_symbol {
                self.next();
                break;
            }

            full_string.push(chr);
            self.next();
        }

        Ok(Token::Str(full_string))
    }
}
