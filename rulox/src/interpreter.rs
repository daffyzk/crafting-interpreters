use crate::{ast::{Binary, Expr, Grouping, Literal, Unary, Value, Visitor}, lox::TokenType};

struct Interpreter {}

impl Interpreter {
    fn evaluate(&self, expr: Expr) -> Expr {
        match expr.clone() {
            Expr::Binary(_) => expr.accept(self),
            Expr::Grouping(_) => expr.accept(self),
            Expr::Unary(_) => expr.accept(self),
            _ => {panic!("evaluate panic")}
        }
    }
}

impl Visitor<Expr> for Interpreter {
    fn visit_binary(&self, binary: &Binary) -> Expr {
        self.evaluate(*binary.left.clone())
    }
    fn visit_grouping(&self, grouping: &Grouping) -> Expr {
        self.evaluate(*grouping.expression.clone())
    }
    fn visit_literal(&self, literal: &Literal) -> Expr {
        Expr::Literal(literal.clone()) // TODO this in theory should return the value but whatever
    }
    fn visit_unary(&self, unary: &Unary) -> Expr { 
        let right: Expr = self.evaluate(*unary.right.clone());
        match unary.operator.type_of {
            TokenType::Minus => {
                match right {
                    Expr::Literal(r) => {
                        match r.value {
                            Value::Integer(i) => { 
                                return Expr::Literal(Literal{ value: Value::Integer(-i)})
                            },
                            Value::Float(f) => {
                                return Expr::Literal(Literal{ value: Value::Float(-f)})
                            },
                            _ => {panic!("value is string or boolean panic")}
                        }
                    },
                    _ => { panic!("right panic") },
                } 
            },
            _ => {panic!("unary panic")}
        }
    }

}
