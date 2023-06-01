use super::ast;
use super::token;

pub struct Parser {
    tokens: Vec<token::Token>,
    cursor: usize,
    errors: Vec<String>,
}

impl Parser {
    // Public
    pub fn new(tokens: Vec<token::Token>) -> Self {
        Parser {
            tokens,
            cursor: 0,
            errors: Vec::with_capacity(5),
        }
    }

    pub fn parse(&mut self) -> Result<ast::Expr, String> {
        let e = self.expression();

        if self.errors.len() == 0 {
            Ok(e)
        } else {
            Err(self.errors.join("\n"))
        }
    }

    // Parse
    fn expression(&mut self) -> ast::Expr {
        self.equality()
    }

    fn equality(&mut self) -> ast::Expr {
        let mut e = self.comparison();

        while self.match_token_types_equality() {
            let op = self.previous();
            let right = self.comparison();
            e = ast::Expr::Binary(Box::new(ast::Binary { left: e, op, right }))
        }

        e
    }

    fn comparison(&mut self) -> ast::Expr {
        let mut e = self.term();

        while self.match_token_types_comparison() {
            let op = self.previous();
            let right = self.term();
            e = ast::Expr::Binary(Box::new(ast::Binary { left: e, op, right }))
        }

        e
    }

    fn term(&mut self) -> ast::Expr {
        let mut e = self.factor();

        while self.match_token_types_term() {
            let op = self.previous();
            let right = self.factor();
            e = ast::Expr::Binary(Box::new(ast::Binary { left: e, op, right }))
        }

        e
    }

    fn factor(&mut self) -> ast::Expr {
        let mut e = self.unary();

        while self.match_token_types_factor() {
            let op = self.previous();
            let right = self.unary();
            e = ast::Expr::Binary(Box::new(ast::Binary { left: e, op, right }))
        }

        e
    }

    fn unary(&mut self) -> ast::Expr {
        if self.match_token_types_unary() {
            let op = self.previous();
            let right = self.unary();
            ast::Expr::Unary(Box::new(ast::Unary { op, right }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> ast::Expr {
        if self.match_token_types_literal() {
            ast::Expr::Literal(Box::new(ast::Literal {
                value: self.previous_value(),
            }))
        } else if self.match_token_types_left_paren() {
            let exp = self.expression();
            self.consume(token::Type::RightParen, "expecting ')'");
            ast::Expr::Grouping(Box::new(ast::Grouping { exp }))
        } else {
            ast::Expr::None
        }
    }

    // Utilities

    fn match_token_types_equality(&mut self) -> bool {
        let t = self.tokens[self.cursor].typ;
        let m = t == token::Type::Equal || t == token::Type::BangEqual;
        if m {
            self.advance();
        }
        m
    }

    fn match_token_types_comparison(&mut self) -> bool {
        let t = self.tokens[self.cursor].typ;
        let m = t == token::Type::BangEqual
            || t == token::Type::Greater
            || t == token::Type::GreaterEqual
            || t == token::Type::Less
            || t == token::Type::LessEqual;
        if m {
            self.advance();
        }
        m
    }

    fn match_token_types_term(&mut self) -> bool {
        let t = self.tokens[self.cursor].typ;
        let m = t == token::Type::Minus || t == token::Type::Plus;
        if m {
            self.advance();
        }
        m
    }

    fn match_token_types_factor(&mut self) -> bool {
        let t = self.tokens[self.cursor].typ;
        let m = t == token::Type::Slash || t == token::Type::Star;
        if m {
            self.advance();
        }
        m
    }

    fn match_token_types_unary(&mut self) -> bool {
        let t = self.tokens[self.cursor].typ;
        let m = t == token::Type::Bang || t == token::Type::Minus;
        if m {
            self.advance();
        }
        m
    }
    fn match_token_types_literal(&mut self) -> bool {
        let t = self.tokens[self.cursor].typ;
        let m = t == token::Type::True
            || t == token::Type::False
            || t == token::Type::Nil
            || t == token::Type::String
            || t == token::Type::Number;
        if m {
            self.advance();
        }
        m
    }

    fn match_token_types_left_paren(&mut self) -> bool {
        let t = self.tokens[self.cursor].typ;
        let m = t == token::Type::LeftParen;
        if m {
            self.advance();
        }
        m
    }

    fn consume(&mut self, token_type: token::Type, msg: &str) {
        let current_token = &self.tokens[self.cursor];

        if current_token.typ == token_type {
            self.advance();
        } else {
            self.errors.push(std::fmt::format(format_args!(
                "line {}: {}",
                current_token.line, msg
            )));
        }
    }

    fn advance(&mut self) -> token::Token {
        if !self.is_at_end() {
            self.cursor += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.cursor].typ == token::Type::EndOfFile
    }

    fn previous(&self) -> token::Token {
        self.tokens[self.cursor - 1].clone()
    }

    fn previous_value(&self) -> token::Value {
        self.tokens[self.cursor - 1].literal.clone()
    }
}
