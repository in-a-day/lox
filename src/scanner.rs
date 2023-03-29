use crate::{
    runner,
    token::{Token, TokenType, LiteralValue},
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::And);
    m.insert("class", TokenType::Class);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::False);
    m.insert("for", TokenType::For);
    m.insert("fun", TokenType::Fun);
    m.insert("if", TokenType::If);
    m.insert("nil", TokenType::Nil);
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("super", TokenType::Super);
    m.insert("this", TokenType::This);
    m.insert("true", TokenType::True);
    m.insert("var", TokenType::Var);
    m.insert("while", TokenType::While);
    m
});

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            match self.advance() {
                // single character
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::SemiColon),
                '*' => self.add_token(TokenType::Star),
                // double
                // ignore comment
                '/' => {
                    if self.is_match('/') {
                        while !self.is_at_end() && Some('\n') != self.peek() {
                            self.advance();
                        }
                    } else if self.is_match('*') {
                        self.block_comment();
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                '!' => {
                    if self.is_match('=') {
                        self.add_token(TokenType::BangEqual);
                    } else {
                        self.add_token(TokenType::Bang);
                    }
                }
                '=' => {
                    if self.is_match('=') {
                        self.add_token(TokenType::EqualEqual);
                    } else {
                        self.add_token(TokenType::Equal);
                    }
                }
                '<' => {
                    if self.is_match('=') {
                        self.add_token(TokenType::LessEqual);
                    } else {
                        self.add_token(TokenType::Less);
                    }
                }
                '>' => {
                    if self.is_match('=') {
                        self.add_token(TokenType::GreaterEqual);
                    } else {
                        self.add_token(TokenType::Greater);
                    }
                }
                // ignore whitespace
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,

                // literal
                '"' => self.string(),
                c if Self::is_digit(c) => self.digital(),
                c if Self::is_alpha(c) => self.identifier(),

                _ => runner::error(self.line, "Unexpected character."),
            }
        }

        self.add_token(TokenType::Eof);
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        (self.current as usize) >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current as usize)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current as usize + 1)
    }

    // find current character, and move current to next
    fn advance(&mut self) -> char {
        let r = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        r
    }

    fn is_match(&mut self, c: char) -> bool {
        match self.peek() {
            Some(v) if v == c => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn is_digit(c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn is_alpha(c: char) -> bool {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
    }

    fn is_alpah_numberic(c: char) -> bool {
        Self::is_digit(c) || Self::is_alpha(c)
    }

    fn digital(&mut self) {
        while Self::is_digit(self.peek().unwrap_or('\0')) {
            self.advance();
        }
        if Some('.') == self.peek() && Self::is_digit(self.peek_next().unwrap_or('\0')) {
            self.advance();
            while Self::is_digit(self.peek().unwrap_or('\0')) {
                self.advance();
            }
        }

        let v = self
            .source
            .get(self.start as usize..self.current as usize)
            .unwrap()
            .parse::<f64>()
            .unwrap();
        self.add_token_with_literal(TokenType::Number, Some(LiteralValue::Nubmer(v)));
    }

    fn string(&mut self) {
        while !self.is_at_end() && self.peek() != Some('"') {
            if self.advance() == '\n' {
                self.line += 1;
            }
        }
        if self.is_at_end() {
            runner::error(self.line, "string unclosed.");
            return;
        }
        // move to the closed "
        self.advance();

        let v = self
            .source
            .get((self.start as usize + 1)..(self.current as usize - 1))
            .unwrap()
            .to_owned();
        self.add_token_with_literal(TokenType::String, Some(LiteralValue::String(v)));
    }

    fn identifier(&mut self) {
        while Self::is_alpah_numberic(self.peek().unwrap_or('\0')) {
            self.advance();
        }

        let v = self
            .source
            .get((self.start as usize)..(self.current as usize))
            .unwrap();
        let token_type = KEYWORDS.get(v).unwrap_or(&TokenType::Identifier).clone();
        // add literal
        match token_type {
            TokenType::Nil => self.add_token_with_literal(token_type, Some(LiteralValue::Nil)),
            TokenType::True => self.add_token_with_literal(token_type, Some(LiteralValue::Bool(true))),
            TokenType::False => self.add_token_with_literal(token_type, Some(LiteralValue::Bool(false))),
            _ => self.add_token(token_type),
        }
    }

    fn block_comment(&mut self) {
        while !self.is_at_end() && !(self.peek() == Some('*') && self.peek_next() == Some('/')) {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            runner::error(self.line, "Block comment unclosed.");
            return;
        }
        // move to */
        self.advance();
        self.advance();
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let lexeme = self
            .source
            .get((self.start as usize)..(self.current as usize))
            .unwrap()
            .to_owned();
        self.tokens.push(Token::new(token_type, lexeme, literal, self.line));
    }
}
