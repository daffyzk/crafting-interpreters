use crate::ast::{Binary, Expr, Grouping, Literal, Unary, Visitor};


pub struct PrettyPrinter;

impl Visitor<String> for PrettyPrinter {
    fn visit_binary(&self, binary: &Binary) -> String {
        self.parenthesize(
            &binary.operator.lexeme.to_string(),
            &[*binary.left.clone(), *binary.right.clone()]
        )
    }

    fn visit_grouping(&self, grouping: &Grouping) -> String {
        self.parenthesize(
            "group",
            &[*grouping.expression.clone()]
        )
    }

    fn visit_literal(&self, literal: &Literal) -> String {
        literal.value.clone().to_string()
    }

    fn visit_unary(&self, unary: &Unary) -> String { 
        self.parenthesize(
            &unary.operator.lexeme.to_string(),
            &[*unary.right.clone()]
        )
    }
}

impl PrettyPrinter {

    pub fn new() -> Self {
        PrettyPrinter{}
    }

    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[Expr]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        builder
    }
}


