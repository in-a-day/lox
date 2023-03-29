use crate::{
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    token::{Token, TokenType}, runner,
};

type Result<T> = std::result::Result<T, ParseErr>;

pub enum ParseErr {
    TokenErr {
        token: Token,
        message: &'static str,
    },
    FooErr,
}

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    pub fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        while self.is_match(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                right,
                operator,
            }));
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        while self.is_match(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                right,
                operator,
            }));
        }

        Ok(expr)
    }

    pub fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                right,
                operator,
            }));
        }

        Ok(expr)
    }

    pub fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while self.is_match(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                right,
                operator,
            }));
        }

        Ok(expr)
    }

    pub fn unary(&mut self) -> Result<Expr> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary(Box::new(UnaryExpr { operator, right })))
        } else {
            self.primary()
        }
    }

    pub fn primary(&mut self) -> Result<Expr> {
        if self.is_match(&[
            TokenType::Number,
            TokenType::String,
            TokenType::Nil,
            TokenType::True,
            TokenType::False,
        ]) {
            return Ok(Expr::Literal(Box::new(LiteralExpr {
                value: self.previous().literal.clone().unwrap(),
            })));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            // first place exception will happen
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Expr::Grouping(Box::new(GroupingExpr { expression: expr })))
        } else {
            Err(self.error(self.peek(), "Expect expression."))
        }
    }

    fn error(&self, token: &Token, message: &'static str) -> ParseErr {
        runner::error_token(token, message);
        ParseErr::TokenErr { token: token.clone(), message }
    }

    fn is_match(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_at_end() && &self.peek().token_type == token_type
    }

    fn consume(&mut self, token_type: TokenType, message: &'static str) -> Result<&Token> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    #[allow(unused)]
    fn syncronize(&mut self) {
        use TokenType::*;
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == SemiColon {
                return;
            }
            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
