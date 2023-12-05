use lazy_static::lazy_static;
use std::{collections::HashMap, usize};

use super::{Scan, ScanError};
use crate::{
    token::{
        Object, Token, TokenType, ONE_CORRESPOND_TWO_CHARACTORS_HASHMAP,
        ONE_OR_TWO_CHARACTORS_HASHMAP, SINGLE_CHARACTOR_HASHMAP,
    },
    Result,
};

pub struct Nothing;

lazy_static! {
    pub static ref WHITESPACE_HASHMAP: HashMap<char, Nothing> = {
        let mut m = HashMap::new();
        m.insert(' ', Nothing);
        m.insert('\r', Nothing);
        m.insert('\t', Nothing);
        m
    };
}

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

    fn explicitly_store(&mut self, token_type: TokenType, start: usize, end: usize) -> Result<()> {
        let lexeme = match self.chars.get(start..end) {
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

    fn store(&mut self, token_type: TokenType) -> Result<()> {
        self.explicitly_store(token_type, self.start, self.current)
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
                '"' => match self.advance_until(|next_char| next_char == '"') {
                    Some(_) => {
                        self.explicitly_store(TokenType::String, self.start + 1, self.current - 1)?;
                        continue 'advance;
                    }
                    None => {
                        return Err(Box::new(ScanError::new(
                            self.line,
                            "Unterminated string.".to_string(),
                        )))
                    }
                },
                // 0-9
                digit if digit.is_digit(10) => {
                    // {0-9}+
                    match self.advance_except(|next_char| !next_char.is_digit(10)) {
                        // {0-9}+\.
                        Some(next_char) if next_char == '.' => {
                            match self.chars.get(self.current + 2) {
                                // {0-9}+\.{0-9}+
                                Some(after_dot) if after_dot.is_digit(10) => {
                                    if self
                                        .advance_except(|next_char| !next_char.is_digit(10))
                                        .is_none()
                                    {
                                        break 'advance;
                                    }
                                }
                                // {0-9}+
                                Some(_) => {}
                                None => break 'advance,
                            }
                        }
                        // {0-9}+
                        Some(_) => {}
                        None => break 'advance,
                    };

                    self.store(TokenType::Number)?;
                    continue 'advance;
                }
                whitespace if WHITESPACE_HASHMAP.get(&whitespace).is_some() => continue 'advance,
                '\n' => {
                    self.line += 1;
                    continue 'advance;
                }
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
