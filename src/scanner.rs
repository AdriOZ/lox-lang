use super::token;

pub struct Scanner {
    source: String,
    tokens: Vec<token::Token>,
}

impl Scanner {
    fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: vec![],
        }
    }
}
