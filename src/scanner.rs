use super::token::*;
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
                    literal: Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                ')' => Token {
                    typ: Type::RightParen,
                    lexeme: None,
                    literal: Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '{' => Token {
                    typ: Type::LeftBrace,
                    lexeme: None,
                    literal: Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '}' => Token {
                    typ: Type::RightBrace,
                    lexeme: None,
                    literal: Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                ',' => Token {
                    typ: Type::Comma,
                    lexeme: None,
                    literal: Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '.' => Token {
                    typ: Type::Dot,
                    lexeme: None,
                    literal: Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                ';' => Token {
                    typ: Type::Semicolon,
                    lexeme: None,
                    literal: Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '+' => Token {
                    typ: Type::Plus,
                    lexeme: None,
                    literal: Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '-' => Token {
                    typ: Type::Minus,
                    lexeme: None,
                    literal: Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '*' => Token {
                    typ: Type::Star,
                    lexeme: None,
                    literal: Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '!' => {
                    if let Some(_) = self.advance_if_match('=') {
                        Token {
                            typ: Type::BangEqual,
                            lexeme: None,
                            literal: Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    } else {
                        Token {
                            typ: Type::Bang,
                            lexeme: None,
                            literal: Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    }
                }
                '=' => {
                    if let Some(_) = self.advance_if_match('=') {
                        Token {
                            typ: Type::EqualEqual,
                            lexeme: None,
                            literal: Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    } else {
                        Token {
                            typ: Type::Equal,
                            lexeme: None,
                            literal: Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    }
                }
                '<' => {
                    if let Some(_) = self.advance_if_match('=') {
                        Token {
                            typ: Type::LessEqual,
                            lexeme: None,
                            literal: Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    } else {
                        Token {
                            typ: Type::Less,
                            lexeme: None,
                            literal: Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    }
                }
                '>' => {
                    if let Some(_) = self.advance_if_match('=') {
                        Token {
                            typ: Type::GreaterEqual,
                            lexeme: None,
                            literal: Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    } else {
                        Token {
                            typ: Type::Greater,
                            lexeme: None,
                            literal: Value::Str(self.get_current_literal()),
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
                            literal: Value::Str(self.get_current_literal()),
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
                        literal: Value::Str(self.get_current_literal()),
                        line: self.line,
                    }
                }
                '0'..='9' => {
                    let mut decimal = false;

                    while let Some(skip) = self.peek() {
                        if skip.is_numeric() {
                            self.advance();
                        } else if skip == '.' && !decimal {
                            self.advance();
                            decimal = true;
                        } else {
                            break;
                        }
                    }
                    Token {
                        typ: Type::Number,
                        lexeme: None,
                        literal: Value::Num(self.get_current_literal().parse::<f64>().unwrap()),
                        line: self.line,
                    }
                }
                _ => {
                    if c == '-' || c.is_alphabetic() {
                        while let Some(skip) = self.peek() {
                            if skip == '-' || skip.is_alphanumeric() {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        let current_literal = self.get_current_literal();
                        let current_type = match current_literal.as_str() {
                            "and" => Type::And,
                            "class" => Type::Class,
                            "else" => Type::Else,
                            "fun" => Type::Fun,
                            "for" => Type::For,
                            "if" => Type::If,
                            "nil" => Type::Nil,
                            "or" => Type::Or,
                            "print" => Type::Print,
                            "return" => Type::Return,
                            "super" => Type::Super,
                            "this" => Type::This,
                            "true" => Type::True,
                            "false" => Type::False,
                            "var" => Type::Var,
                            "while" => Type::While,
                            _ => Type::Identifier,
                        };
                        Token {
                            typ: current_type,
                            lexeme: None,
                            literal: Value::Str(current_literal),
                            line: self.line,
                        }
                    } else {
                        Token {
                            typ: Type::EndOfFile,
                            lexeme: None,
                            literal: Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    }
                }
            }
        } else {
            Token {
                typ: Type::EndOfFile,
                lexeme: None,
                literal: Value::None,
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

    fn get_current_literal(&self) -> String {
        String::from_iter((&self.source[self.start..self.cursor]).iter())
            .trim()
            .replace('"', "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let mut scanner = Scanner::new(&"".to_string());
        let tokens = scanner.parse();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].typ, Type::EndOfFile);
    }

    #[test]
    fn whitespace_string() {
        let mut scanner = Scanner::new(&"        ".to_string());
        let tokens = scanner.parse();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].typ, Type::EndOfFile);
    }

    #[test]
    fn single_characters() {
        let mut scanner = Scanner::new(&"(){},.-+;*".to_string());
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 11);

        let expected: Vec<Type> = vec![
            Type::LeftParen,
            Type::RightParen,
            Type::LeftBrace,
            Type::RightBrace,
            Type::Comma,
            Type::Dot,
            Type::Minus,
            Type::Plus,
            Type::Semicolon,
            Type::Star,
            Type::EndOfFile,
        ];
        assert_eq!(
            expected,
            tokens.iter().map(|v| v.typ).collect::<Vec<Type>>()
        );
    }

    #[test]
    fn one_or_two_characters() {
        let mut scanner = Scanner::new(&"! != = == > >= < <=".to_string());
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 9);

        let expected: Vec<Type> = vec![
            Type::Bang,
            Type::BangEqual,
            Type::Equal,
            Type::EqualEqual,
            Type::Greater,
            Type::GreaterEqual,
            Type::Less,
            Type::LessEqual,
            Type::EndOfFile,
        ];
        assert_eq!(
            expected,
            tokens.iter().map(|v| v.typ).collect::<Vec<Type>>()
        );
    }

    #[test]
    fn keywords() {
        let mut scanner = Scanner::new(
            &"and class else fun for if nil or print return super this true false var while"
                .to_string(),
        );
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 17);

        let expected: Vec<Type> = vec![
            Type::And,
            Type::Class,
            Type::Else,
            Type::Fun,
            Type::For,
            Type::If,
            Type::Nil,
            Type::Or,
            Type::Print,
            Type::Return,
            Type::Super,
            Type::This,
            Type::True,
            Type::False,
            Type::Var,
            Type::While,
            Type::EndOfFile,
        ];
        assert_eq!(
            expected,
            tokens.iter().map(|v| v.typ).collect::<Vec<Type>>()
        );
    }
}
