use super::token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
    pub left: Expr,
    pub op: token::Token,
    pub right: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    pub exp: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub value: token::Value,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
    pub op: token::Token,
    pub right: Expr,
}
