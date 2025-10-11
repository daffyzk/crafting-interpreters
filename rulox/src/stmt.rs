use crate::ast::Expr;


pub trait Visitor<T> {
    fn visit_expression(&self, expr: Expr) -> T;
    fn visit_print(&self, print: Expr) -> T;
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expr: Expr,
}

impl Expression {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}

#[derive(Debug, Clone)]
pub struct Print {
    pub print: Expr,
}

impl Print {
    pub fn new(print: Expr) -> Self {
        Self { print }
    }
}
