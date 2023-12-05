use crate::{
    token::{
        Object, Token, TokenType, ONE_CORRESPOND_TWO_CHARACTORS_HASHMAP,
        ONE_OR_TWO_CHARACTORS_HASHMAP, SINGLE_CHARACTOR_HASHMAP,
    },
    Result,
};

use super::{Scan, ScanError};

pub struct Scanner {
    chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            chars: Vec::from_iter(source.chars()),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn store(&mut self, token_type: TokenType) -> Result<()> {
        let lexeme = match self.chars.get(self.start..self.current) {
            Some(chars) => String::from_iter(chars),
            None => {
                return Err(Box::new(ScanError::new(
                    self.line,
                    String::from("out range"),
                )))
            }
        };
        self.tokens
            .push(Token::new(token_type, lexeme, Object {}, self.line));
        Ok(())
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

impl Scan for Scanner {
    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    fn accumulate(&mut self) {
        self.current += 1;
    }

    fn scan(&mut self) -> Result<()> {
        'advance: while let Some(char) = self.advance() {
            self.start = self.current;

            if let Some(token_type) = SINGLE_CHARACTOR_HASHMAP.get(&char) {
                self.store(token_type.clone())?;
                continue 'advance;
            };

            if let Some(token_type) = ONE_OR_TWO_CHARACTORS_HASHMAP.get(&char) {
                match self.peek_or_advance(|next_char| next_char == '=') {
                    Some(is_equal) if is_equal => {
                        match ONE_CORRESPOND_TWO_CHARACTORS_HASHMAP.get(token_type) {
                            Some(_) => self.store(token_type.clone())?,
                            None => {
                                return Err(Box::new(ScanError::new(
                                    self.line,
                                    "no corresponding token".to_string(),
                                )))
                            }
                        }
                    }
                    Some(_) => {
                        self.store(token_type.clone())?;
                    }
                    None => break 'advance,
                };
                continue 'advance;
            }

            match char {
                '/' => match self.peek_or_advance(|next_char| next_char == '/') {
                    Some(is_equal) if is_equal => {
                        self.advance_until(|next_char| next_char == '\n');
                        continue 'advance;
                    }
                    Some(_) => {
                        self.store(TokenType::Slash)?;
                        continue 'advance;
                    }
                    None => break 'advance,
                },
                ' ' => continue 'advance,
                other => {
                    return Err(Box::new(ScanError::new(
                        self.line,
                        format!("other char: {}, error!", other),
                    )));
                }
            }
        }

        self.store(TokenType::Eof)?;

        Ok(())
    }
}
