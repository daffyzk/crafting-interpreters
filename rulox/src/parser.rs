use std::sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex};

use crate::{ast::{Binary, Expr, Grouping, Literal, Unary, Value}, lox::{Lox, Token, TokenType}};

pub struct Parser {
    lox:         Arc<Mutex<Lox>>,
    pub tokens:  Vec<Token>,
    pub current: Arc<AtomicUsize>,
}

impl Parser {

    pub fn new(lox: Arc<Mutex<Lox>>, tokens: Vec<Token>) -> Self {
        Parser {
            lox,
            tokens,
            current: Arc::new(AtomicUsize::new(0usize))
        } 
    }

    pub fn parse(&self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&self) -> Result<Expr, ParseError> {
        let mut expr: Expr = match self.comparison() {
            Ok(v) => {v},
            Err(e)=> {return Err(e)},
        };
        loop {
            if !self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {break};
            let operator: Token = self.previous();
            let right: Expr = match self.comparison() {
                Ok(v) => {v},
                Err(e)=> {return Err(e)},
            };
            expr = Binary::new(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr); 
    }

    fn match_types(&self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {return false};
        let p = self.peek();
        matches!(p.type_of, token_type)
    }
    
    fn advance(&self) -> Token {
        if !self.is_at_end() {self.current.fetch_add(1usize, Ordering::Relaxed);};
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().type_of, TokenType::Eof) 
    }

    fn peek(&self) -> Token {
        match self.tokens.get(self.current.clone().load(Ordering::Relaxed)) {
            Some(t) => {t.clone()},
            None => panic!("Could not peek into current token"),
        }
    }

    fn previous(&self) -> Token {
        match self.tokens.get(self.current.clone().load(Ordering::Relaxed) - 1usize) {
            Some(t) => {t.clone()},
            None => panic!("Could not peek into previous token"),
        }
    }
    
    fn comparison(&self) -> Result<Expr, ParseError> {
        let mut expr: Expr = match self.term() {
            Ok(v) => {v},
            Err(e)=> {return Err(e)},
        };

        loop {
            if !self.match_types(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {return Ok(expr)};
            let operator: Token = self.previous();
            let right: Expr = match self.term() {
                Ok(v) => {v},
                Err(e)=> {return Err(e)},
            };
            expr = Binary::new(Box::new(expr), operator, Box::new(right));
        }
    }
    
    fn term(&self) -> Result<Expr, ParseError>  {
        let mut expr: Expr = match self.factor() {
            Ok(v) => {v},
            Err(e)=> {return Err(e)},
        };
        loop {
            if !self.match_types(vec![TokenType::Minus, TokenType::Plus]) {return Ok(expr)};
            let operator: Token = self.previous();
            let right: Expr = match self.factor() {
                Ok(v) => {v},
                Err(e)=> {return Err(e)},
            };
            expr = Binary::new(Box::new(expr), operator, Box::new(right));
        }
    }

    fn factor(&self) -> Result<Expr, ParseError>  {
        let mut expr: Expr = match self.unary() {
            Ok(v) => {v},
            Err(e)=> {return Err(e)},
        };
        loop {
            if !self.match_types(vec![TokenType::Slash, TokenType::Star]) {return Ok(expr)};
            let operator: Token = self.previous();
            let right: Expr = match self.unary() {
                Ok(v) => {v},
                Err(e)=> {return Err(e)},
            };
            expr = Binary::new(Box::new(expr), operator, Box::new(right));
        } 
    }

    fn unary(&self) -> Result<Expr, ParseError> {
        if self.match_types(vec![]) {
            let operator: Token = self.previous();
            let right: Expr = match self.unary() {
                Ok(v) => {v},
                Err(e)=> {return Err(e)},
            };
            return Ok(Unary::new(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&self) -> Result<Expr, ParseError> {
        if self.match_types(vec![TokenType::False]) {return Ok(Literal::new(Value::Boolean(false)))};
        if self.match_types(vec![TokenType::True])  {return Ok(Literal::new(Value::Boolean(true)))};
        if self.match_types(vec![TokenType::Nil]) {return Ok(Literal::new(Value::Null))};
        if self.match_types(vec![TokenType::Number, TokenType::String]) {
            return Ok(Literal::new(self.previous().literal))
        }
        if self.match_types(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            match self.consume(TokenType::RightParen, "Expected ')' after expression."){
                Ok(_) => {},
                Err(e) => {return Err(e)}
            };
            match expr {
                Ok(e) => return Ok(Grouping::new(Box::new(e))),
                Err(e) => return Err(e)
            }
        }
        Err(ParseError("Expected expression.".to_string()))
    }

    fn consume(&self, token_type: TokenType, err: &str) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance())
        }
        Err(self.error(self.peek(), err)) 
    }
    
    fn error(&self, token: Token, message: &str) -> ParseError {
        self.lox.lock().unwrap().token_error(token, message);
        ParseError(message.into())
    }

    fn synchronize(&self) {
        self.advance();
        
        loop {
            if self.is_at_end() { break;}
            let t = self.peek().type_of;
            match t {
                TokenType::Class  => {},
                TokenType::Fun    => {},
                TokenType::Var    => {},
                TokenType::For    => {},
                TokenType::If     => {},
                TokenType::While  => {},
                TokenType::Print  => {},
                TokenType::Return => break,
                _ => {}
            }

            self.advance();
        }
    }
}

#[derive(Debug)] pub struct ParseError(String);
