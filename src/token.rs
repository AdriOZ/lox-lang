use std::option::Option;

#[derive(Debug)]
pub enum Type {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    //Keywords
    And,
    Class,
    Else,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    False,
    Var,
    While,

    EndOfFile,
}

#[derive(Debug)]
pub enum Value {
    Str(String),
    Bool(bool),
    Num(f64),
    None,
}

#[derive(Debug)]
pub struct Token {
    pub typ: Type,
    pub lexeme: Option<String>,
    pub literal: Value,
    pub line: usize,
}
