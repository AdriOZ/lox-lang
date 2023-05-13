use super::token::{Token, Type};
use std::option::Option;

pub struct Scanner {
    source: Vec<char>,

    cursor: usize,
    line: usize,
    end: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let end = source.len();

        Scanner {
            source: source.chars().collect(),
            cursor: 0,
            line: 1,
            end,
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
        }
        result
    }

    fn next(&mut self) -> Token {
        // Skip whitespace
        while self.source[self.cursor].is_whitespace() && self.cursor < self.end {
            self.cursor += 1;

            if self.source[self.cursor] == '\n' {
                self.line += 1;
            }
        }

        if self.cursor >= self.end {
            Token {
                typ: Type::EndOfFile,
                lexeme: Option::None,
                literal: Option::None,
                line: self.line,
            }
        } else {
            match self.source[self.cursor] {
                '(' => Token {
                    typ: Type::LeftParen,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
                ')' => Token {
                    typ: Type::RightParen,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
                '{' => Token {
                    typ: Type::LeftBrace,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
                '}' => Token {
                    typ: Type::RightBrace,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
                ',' => Token {
                    typ: Type::Comma,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
                '.' => Token {
                    typ: Type::Dot,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
                ';' => Token {
                    typ: Type::Semicolon,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
                '+' => Token {
                    typ: Type::Plus,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
                '-' => Token {
                    typ: Type::Minus,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
                '*' => Token {
                    typ: Type::Star,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
                _ => Token {
                    typ: Type::EndOfFile,
                    lexeme: Option::None,
                    literal: Option::None,
                    line: self.line,
                },
            }
        }
    }
}
