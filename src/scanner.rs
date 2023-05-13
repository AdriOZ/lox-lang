use super::token::{Token, Type};
use std::option::Option;

pub struct Scanner {
    source: Vec<char>,

    cursor: usize,
    start: usize,
    end: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &String) -> Self {
        let end = source.len();

        Scanner {
            source: source.chars().collect(),
            cursor: 0,
            start: 0,
            end,
            line: 1,
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::with_capacity(self.end / 2);
        let mut tok = self.next();
        let mut has_next = true;

        while has_next {
            has_next = match tok.typ {
                Type::EndOfFile => false,
                _ => true,
            };
            result.push(tok);
            tok = self.next();
            self.start = self.cursor;
        }
        result
    }

    fn next(&mut self) -> Token {
        // Skip whitespace
        let mut current = self.advance();

        while let Some(c) = current {
            if c.is_whitespace() {
                current = self.advance();
            } else {
                break;
            }
        }

        if let Some(c) = current {
            match c {
                '(' => Token {
                    typ: Type::LeftParen,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
                ')' => Token {
                    typ: Type::RightParen,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
                '{' => Token {
                    typ: Type::LeftBrace,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
                '}' => Token {
                    typ: Type::RightBrace,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
                ',' => Token {
                    typ: Type::Comma,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
                '.' => Token {
                    typ: Type::Dot,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
                ';' => Token {
                    typ: Type::Semicolon,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
                '+' => Token {
                    typ: Type::Plus,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
                '-' => Token {
                    typ: Type::Minus,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
                '*' => Token {
                    typ: Type::Star,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
                _ => Token {
                    typ: Type::EndOfFile,
                    lexeme: None,
                    literal: self.get_current_literal(),
                    line: self.line,
                },
            }
        } else {
            Token {
                typ: Type::EndOfFile,
                lexeme: None,
                literal: None,
                line: self.line,
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.end
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            let current = self.source[self.cursor];

            if current == '\n' {
                self.line += 1;
            }
            self.cursor += 1;
            Some(current)
        }
    }

    fn get_current_literal(&self) -> Option<String> {
        Some(
            String::from_iter((&self.source[self.start..self.cursor]).iter())
                .trim()
                .to_string(),
        )
    }
}
