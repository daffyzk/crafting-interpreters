use crate::Token;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Float(f64),
    Integer(u32),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Float(f) => f.to_string(),
            Value::Integer(i) => i.to_string(),
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary(&self, binary: &Binary) -> T;
    fn visit_grouping(&self, grouping: &Grouping) -> T;
    fn visit_literal(&self, literal: &Literal) -> T;
    fn visit_unary(&self, unary: &Unary) -> T;
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Unary(unary) => visitor.visit_unary(unary),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Expr {
        Expr::Binary(Binary { left, operator, right })
    }
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Expr {
        Expr::Grouping(Grouping { expression })
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: Value,
}

impl Literal {
    pub fn new(value: Value) -> Expr {
        Expr::Literal(Literal { value })
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Expr {
        Expr::Unary(Unary { operator, right })
    }
}

// this sucks
struct Parser {
    pub tokens: Vec<Token>,
    pub current: u32,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0
        } 
    }
}


// Grouping

// use std::env;
// const args: Vec<String> = env::args().collect();
// let lox: Lox = Lox{};
//
// lox::main(lox, args);
