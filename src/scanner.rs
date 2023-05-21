use super::token;
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

    pub fn parse(&mut self) -> Vec<token::Token> {
        let mut result: Vec<token::Token> = Vec::with_capacity(self.end / 2);
        let mut tok = self.next();
        let mut has_next = true;

        while has_next {
            has_next = match tok.typ {
                token::Type::EndOfFile => false,
                _ => true,
            };
            result.push(tok);
            tok = self.next();
        }
        result
    }

    fn next(&mut self) -> token::Token {
        self.skip_whitespace();
        self.start = self.cursor;
        let current = self.advance();

        if let Some(c) = current {
            match c {
                '(' => token::Token {
                    typ: token::Type::LeftParen,
                    lexeme: None,
                    literal: token::Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                ')' => token::Token {
                    typ: token::Type::RightParen,
                    lexeme: None,
                    literal: token::Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '{' => token::Token {
                    typ: token::Type::LeftBrace,
                    lexeme: None,
                    literal: token::Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '}' => token::Token {
                    typ: token::Type::RightBrace,
                    lexeme: None,
                    literal: token::Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                ',' => token::Token {
                    typ: token::Type::Comma,
                    lexeme: None,
                    literal: token::Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '.' => token::Token {
                    typ: token::Type::Dot,
                    lexeme: None,
                    literal: token::Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                ';' => token::Token {
                    typ: token::Type::Semicolon,
                    lexeme: None,
                    literal: token::Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '+' => token::Token {
                    typ: token::Type::Plus,
                    lexeme: None,
                    literal: token::Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '-' => token::Token {
                    typ: token::Type::Minus,
                    lexeme: None,
                    literal: token::Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '*' => token::Token {
                    typ: token::Type::Star,
                    lexeme: None,
                    literal: token::Value::Str(self.get_current_literal()),
                    line: self.line,
                },
                '!' => {
                    if let Some(_) = self.advance_if_match('=') {
                        token::Token {
                            typ: token::Type::BangEqual,
                            lexeme: None,
                            literal: token::Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    } else {
                        token::Token {
                            typ: token::Type::Bang,
                            lexeme: None,
                            literal: token::Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    }
                }
                '=' => {
                    if let Some(_) = self.advance_if_match('=') {
                        token::Token {
                            typ: token::Type::EqualEqual,
                            lexeme: None,
                            literal: token::Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    } else {
                        token::Token {
                            typ: token::Type::Equal,
                            lexeme: None,
                            literal: token::Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    }
                }
                '<' => {
                    if let Some(_) = self.advance_if_match('=') {
                        token::Token {
                            typ: token::Type::LessEqual,
                            lexeme: None,
                            literal: token::Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    } else {
                        token::Token {
                            typ: token::Type::Less,
                            lexeme: None,
                            literal: token::Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    }
                }
                '>' => {
                    if let Some(_) = self.advance_if_match('=') {
                        token::Token {
                            typ: token::Type::GreaterEqual,
                            lexeme: None,
                            literal: token::Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    } else {
                        token::Token {
                            typ: token::Type::Greater,
                            lexeme: None,
                            literal: token::Value::Str(self.get_current_literal()),
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
                        token::Token {
                            typ: token::Type::Slash,
                            lexeme: None,
                            literal: token::Value::Str(self.get_current_literal()),
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
                    token::Token {
                        typ: token::Type::String,
                        lexeme: None,
                        literal: token::Value::Str(self.get_current_literal()),
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
                    token::Token {
                        typ: token::Type::Number,
                        lexeme: None,
                        literal: token::Value::Num(
                            self.get_current_literal().parse::<f64>().unwrap(),
                        ),
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
                            "and" => token::Type::And,
                            "class" => token::Type::Class,
                            "else" => token::Type::Else,
                            "fun" => token::Type::Fun,
                            "for" => token::Type::For,
                            "if" => token::Type::If,
                            "nil" => token::Type::Nil,
                            "or" => token::Type::Or,
                            "print" => token::Type::Print,
                            "return" => token::Type::Return,
                            "super" => token::Type::Super,
                            "this" => token::Type::This,
                            "true" => token::Type::True,
                            "false" => token::Type::False,
                            "var" => token::Type::Var,
                            "while" => token::Type::While,
                            _ => token::Type::Identifier,
                        };
                        let current_value = match current_type {
                            token::Type::True => token::Value::Bool(true),
                            token::Type::False => token::Value::Bool(false),
                            _ => token::Value::Str(current_literal),
                        };
                        token::Token {
                            typ: current_type,
                            lexeme: None,
                            literal: current_value,
                            line: self.line,
                        }
                    } else {
                        token::Token {
                            typ: token::Type::EndOfFile,
                            lexeme: None,
                            literal: token::Value::Str(self.get_current_literal()),
                            line: self.line,
                        }
                    }
                }
            }
        } else {
            token::Token {
                typ: token::Type::EndOfFile,
                lexeme: None,
                literal: token::Value::None,
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
        assert_eq!(tokens[0].typ, token::Type::EndOfFile);
    }

    #[test]
    fn whitespace_string() {
        let mut scanner = Scanner::new(&"        ".to_string());
        let tokens = scanner.parse();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].typ, token::Type::EndOfFile);
    }

    #[test]
    fn single_characters() {
        let mut scanner = Scanner::new(&"(){},.-+;*".to_string());
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 11);

        let expected: Vec<token::Type> = vec![
            token::Type::LeftParen,
            token::Type::RightParen,
            token::Type::LeftBrace,
            token::Type::RightBrace,
            token::Type::Comma,
            token::Type::Dot,
            token::Type::Minus,
            token::Type::Plus,
            token::Type::Semicolon,
            token::Type::Star,
            token::Type::EndOfFile,
        ];
        assert_eq!(
            expected,
            tokens.iter().map(|v| v.typ).collect::<Vec<token::Type>>()
        );
    }

    #[test]
    fn one_or_two_characters() {
        let mut scanner = Scanner::new(&"! != = == > >= < <=".to_string());
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 9);

        let expected: Vec<token::Type> = vec![
            token::Type::Bang,
            token::Type::BangEqual,
            token::Type::Equal,
            token::Type::EqualEqual,
            token::Type::Greater,
            token::Type::GreaterEqual,
            token::Type::Less,
            token::Type::LessEqual,
            token::Type::EndOfFile,
        ];
        assert_eq!(
            expected,
            tokens.iter().map(|v| v.typ).collect::<Vec<token::Type>>()
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

        let expected: Vec<token::Type> = vec![
            token::Type::And,
            token::Type::Class,
            token::Type::Else,
            token::Type::Fun,
            token::Type::For,
            token::Type::If,
            token::Type::Nil,
            token::Type::Or,
            token::Type::Print,
            token::Type::Return,
            token::Type::Super,
            token::Type::This,
            token::Type::True,
            token::Type::False,
            token::Type::Var,
            token::Type::While,
            token::Type::EndOfFile,
        ];
        assert_eq!(
            expected,
            tokens.iter().map(|v| v.typ).collect::<Vec<token::Type>>()
        );
    }

    #[test]
    fn string_literals() {
        let mut scanner = Scanner::new(
            &"\"This is a string literal\" \"This is another string literal without ending quotes"
                .to_string(),
        );
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 3);

        let expected_types: Vec<token::Type> = vec![
            token::Type::String,
            token::Type::String,
            token::Type::EndOfFile,
        ];
        assert_eq!(
            expected_types,
            tokens.iter().map(|v| v.typ).collect::<Vec<token::Type>>()
        );

        if let token::Value::Str(v) = &tokens[0].literal {
            assert_eq!("This is a string literal", v);
        }
        if let token::Value::Str(v) = &tokens[1].literal {
            assert_eq!("This is another string literal without ending quotes", v);
        }
    }

    #[test]
    fn numbers() {
        let mut scanner = Scanner::new(&"1 1.1 1..1".to_string());
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 6);

        let expected_types: Vec<token::Type> = vec![
            token::Type::Number,
            token::Type::Number,
            token::Type::Number,
            token::Type::Dot,
            token::Type::Number,
            token::Type::EndOfFile,
        ];
        assert_eq!(
            expected_types,
            tokens.iter().map(|v| v.typ).collect::<Vec<token::Type>>()
        );

        if let token::Value::Num(v) = tokens[0].literal {
            assert_eq!(1.0 as f64, v);
        }
        if let token::Value::Num(v) = tokens[1].literal {
            assert_eq!(1.1 as f64, v);
        }
        if let token::Value::Num(v) = tokens[2].literal {
            assert_eq!(1.0 as f64, v);
        }
        if let token::Value::Num(v) = tokens[4].literal {
            assert_eq!(1.0 as f64, v);
        }
    }

    #[test]
    fn expressions() {
        let mut scanner = Scanner::new(&"var v1 = true; var v2 = 1.1;".to_string());
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 11);

        let expected_types: Vec<token::Type> = vec![
            token::Type::Var,
            token::Type::Identifier,
            token::Type::Equal,
            token::Type::True,
            token::Type::Semicolon,
            token::Type::Var,
            token::Type::Identifier,
            token::Type::Equal,
            token::Type::Number,
            token::Type::Semicolon,
            token::Type::EndOfFile,
        ];
        assert_eq!(
            expected_types,
            tokens.iter().map(|v| v.typ).collect::<Vec<token::Type>>()
        );

        if let token::Value::Str(v) = &tokens[1].literal {
            assert_eq!("v1", v);
        }
        if let token::Value::Bool(v) = tokens[3].literal {
            assert_eq!(true, v);
        }
        if let token::Value::Str(v) = &tokens[6].literal {
            assert_eq!("v2", v);
        }
        if let token::Value::Num(v) = tokens[8].literal {
            assert_eq!(1.1 as f64, v);
        }
    }

    #[test]
    fn functions() {
        let mut scanner = Scanner::new(&"fun main() {}".to_string());
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 7);

        let expected_types: Vec<token::Type> = vec![
            token::Type::Fun,
            token::Type::Identifier,
            token::Type::LeftParen,
            token::Type::RightParen,
            token::Type::LeftBrace,
            token::Type::RightBrace,
            token::Type::EndOfFile,
        ];
        assert_eq!(
            expected_types,
            tokens.iter().map(|v| v.typ).collect::<Vec<token::Type>>()
        );

        if let token::Value::Str(v) = &tokens[1].literal {
            assert_eq!("main", v);
        }
    }

    #[test]
    fn classes() {
        let mut scanner = Scanner::new(&"class Car {}".to_string());
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 5);

        let expected_types: Vec<token::Type> = vec![
            token::Type::Class,
            token::Type::Identifier,
            token::Type::LeftBrace,
            token::Type::RightBrace,
            token::Type::EndOfFile,
        ];
        assert_eq!(
            expected_types,
            tokens.iter().map(|v| v.typ).collect::<Vec<token::Type>>()
        );

        if let token::Value::Str(v) = &tokens[1].literal {
            assert_eq!("Car", v);
        }
    }

    #[test]
    fn full_program() {
        let mut scanner = Scanner::new(
            &r#"
            
                class Person {
                    Person(name, age, married) {
                        this.name = name;
                        this.age = age;
                        this.married = married;
                    }
                }

                fun main() {
                    var p = Person("Bob", 30, false);
                    print p.name;
                    print p.age;
                    print p.married;
                }

            "#
            .to_string(),
        );
        let tokens = scanner.parse();

        assert_eq!(tokens.len(), 66);

        let expected_types: Vec<token::Type> = vec![
            // Line
            token::Type::Class,
            token::Type::Identifier,
            token::Type::LeftBrace,
            // Line
            token::Type::Identifier,
            token::Type::LeftParen,
            token::Type::Identifier,
            token::Type::Comma,
            token::Type::Identifier,
            token::Type::Comma,
            token::Type::Identifier,
            token::Type::RightParen,
            token::Type::LeftBrace,
            // Line
            token::Type::This,
            token::Type::Dot,
            token::Type::Identifier,
            token::Type::Equal,
            token::Type::Identifier,
            token::Type::Semicolon,
            // Line
            token::Type::This,
            token::Type::Dot,
            token::Type::Identifier,
            token::Type::Equal,
            token::Type::Identifier,
            token::Type::Semicolon,
            // Line
            token::Type::This,
            token::Type::Dot,
            token::Type::Identifier,
            token::Type::Equal,
            token::Type::Identifier,
            token::Type::Semicolon,
            // Line
            token::Type::RightBrace,
            // Line
            token::Type::RightBrace,
            // Line
            token::Type::Fun,
            token::Type::Identifier,
            token::Type::LeftParen,
            token::Type::RightParen,
            token::Type::LeftBrace,
            // Line
            token::Type::Var,
            token::Type::Identifier,
            token::Type::Equal,
            token::Type::Identifier,
            token::Type::LeftParen,
            token::Type::String,
            token::Type::Comma,
            token::Type::Number,
            token::Type::Comma,
            token::Type::False,
            token::Type::RightParen,
            token::Type::Semicolon,
            // Line
            token::Type::Print,
            token::Type::Identifier,
            token::Type::Dot,
            token::Type::Identifier,
            token::Type::Semicolon,
            // Line
            token::Type::Print,
            token::Type::Identifier,
            token::Type::Dot,
            token::Type::Identifier,
            token::Type::Semicolon,
            // Line
            token::Type::Print,
            token::Type::Identifier,
            token::Type::Dot,
            token::Type::Identifier,
            token::Type::Semicolon,
            // Line
            token::Type::RightBrace,
            // Line
            token::Type::EndOfFile,
        ];
        assert_eq!(
            expected_types,
            tokens.iter().map(|v| v.typ).collect::<Vec<token::Type>>()
        );
    }
}
