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
        Scanner {
            source: source.chars().collect(),
            cursor: 0,
            start: 0,
            end: source.len(),
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
        }
        result
    }

    fn next(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.cursor;
        let current = self.advance();

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
                '!' => {
                    if let Some(_) = self.advance_if_match('=') {
                        Token {
                            typ: Type::BangEqual,
                            lexeme: None,
                            literal: self.get_current_literal(),
                            line: self.line,
                        }
                    } else {
                        Token {
                            typ: Type::Bang,
                            lexeme: None,
                            literal: self.get_current_literal(),
                            line: self.line,
                        }
                    }
                }
                '=' => {
                    if let Some(_) = self.advance_if_match('=') {
                        Token {
                            typ: Type::EqualEqual,
                            lexeme: None,
                            literal: self.get_current_literal(),
                            line: self.line,
                        }
                    } else {
                        Token {
                            typ: Type::Equal,
                            lexeme: None,
                            literal: self.get_current_literal(),
                            line: self.line,
                        }
                    }
                }
                '<' => {
                    if let Some(_) = self.advance_if_match('=') {
                        Token {
                            typ: Type::LessEqual,
                            lexeme: None,
                            literal: self.get_current_literal(),
                            line: self.line,
                        }
                    } else {
                        Token {
                            typ: Type::Less,
                            lexeme: None,
                            literal: self.get_current_literal(),
                            line: self.line,
                        }
                    }
                }
                '>' => {
                    if let Some(_) = self.advance_if_match('=') {
                        Token {
                            typ: Type::GreaterEqual,
                            lexeme: None,
                            literal: self.get_current_literal(),
                            line: self.line,
                        }
                    } else {
                        Token {
                            typ: Type::Greater,
                            lexeme: None,
                            literal: self.get_current_literal(),
                            line: self.line,
                        }
                    }
                }
                '/' => {
                    if let Some(_) = self.advance_if_match('/') {
                        while let Some(skip) = self.peek() {
                            if skip != '\n' && skip != '\r' {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        self.next()
                    } else {
                        Token {
                            typ: Type::Slash,
                            lexeme: None,
                            literal: self.get_current_literal(),
                            line: self.line,
                        }
                    }
                }
                '"' => {
                    self.start = self.cursor;
                    while let Some(skip) = self.peek() {
                        if skip != '"' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.advance();
                    Token {
                        typ: Type::String,
                        lexeme: None,
                        literal: self.get_current_literal(),
                        line: self.line,
                    }
                }
                '0'..='9' => {
                    while let Some(skip) = self.peek() {
                        if skip.is_numeric() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    Token {
                        typ: Type::Number,
                        lexeme: None,
                        literal: self.get_current_literal(),
                        line: self.line,
                    }
                }
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

    fn advance_if_match(&mut self, c: char) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            let current = self.source[self.cursor];

            if current == c {
                self.cursor += 1;
                Some(current)
            } else {
                None
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.source[self.cursor].is_whitespace() {
            if self.source[self.cursor] == '\n' {
                self.line += 1;
            }
            self.cursor += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.cursor])
        }
    }

    fn get_current_literal(&self) -> Option<String> {
        Some(
            String::from_iter((&self.source[self.start..self.cursor]).iter())
                .trim()
                .replace('"', ""),
        )
    }
}
