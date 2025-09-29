use crate::{ast::{Binary, Expr, Grouping, Literal, Unary, Value, Visitor}, lox::TokenType, runtime_error::RuntimeError};

struct Interpreter {}

impl Interpreter {
    fn evaluate(&self, expr: Expr) -> Expr {
        match expr.clone() {
            Expr::Binary(_) => expr.accept(self),
            Expr::Grouping(_) => expr.accept(self),
            Expr::Unary(_) => expr.accept(self),
            Expr::Literal(_) => expr.accept(self),
        }
    }
    fn is_truthy(value: Value) -> bool {
        match value {
            Value::Null => { return false },
            Value::Boolean(i) => { return i },
            _ => { return true }
        }
    }
    fn is_equal(tuplio: (Value, Value)) -> bool {
        match tuplio {
           (Value::Null, Value::Null) => { true },
           (Value::Null, _) => { false },
           (Value::Integer(v), Value::Integer(w)) => { v == w },
           (Value::Float(v), Value::Float(w)) => { v == w },
           (Value::String(v), Value::String(w)) => { v == w },
           (Value::Boolean(v), Value::Boolean(w)) => { v == w },
           _ => {panic!("is equal different unexpected value")},
        }
    }
    fn match_binaries(left: Expr, right: Expr) -> (Value, Value) {
        let lv = match left {
            Expr::Literal(v) => {v.value}
            _ => {panic!("binary minus left expected literal")}
        };
        let rv = match right {
            Expr::Literal(v) => {v.value}
            _ => {panic!("binary minus right expected literal")}
        };
        return (lv, rv)
    }

    fn check_number_operands(token: TokenType, left: Expr, right: Expr) -> Expr {
        let value_type: &str = match token {
            TokenType::Greater | 
            TokenType::GreaterEqual | 
            TokenType::Less |
            TokenType::LessEqual
                => "compare",
            TokenType::Slash => { "divide" },
            TokenType::Star => { "multiply" },
            TokenType::Minus => { "subtract" },
            TokenType::Plus => { "add" },
            _ => "!!undefined operation!!"
        };
        return match Interpreter::match_binaries(left, right) {
            (Value::Integer(v), Value::Integer(w)) => { 
                match token {
                    TokenType::Greater => { Literal::new(Value::Boolean(v>w)) },
                    TokenType::GreaterEqual => { Literal::new(Value::Boolean(v>=w)) },
                    TokenType::Less => { Literal::new(Value::Boolean(v<w)) },
                    TokenType::LessEqual => { Literal::new(Value::Boolean(v<=w)) },
                    TokenType::Slash => { Literal::new(Value::Integer(v/w)) },
                    TokenType::Star => { Literal::new(Value::Integer(v*w)) },
                    TokenType::Minus => { Literal::new(Value::Integer(v-w)) },
                    TokenType::Plus => { Literal::new(Value::Integer(v+w)) },
                    _ => {panic!("binary error: unexpected token type: {}", value_type)},
                }

            },
            (Value::Float(v), Value::Float(w)) => { 
                match token {
                    TokenType::Greater => { Literal::new(Value::Boolean(v>w)) },
                    TokenType::GreaterEqual => { Literal::new(Value::Boolean(v>=w)) },
                    TokenType::Less => { Literal::new(Value::Boolean(v<w)) },
                    TokenType::LessEqual => { Literal::new(Value::Boolean(v<=w)) },
                    TokenType::Slash => { Literal::new(Value::Float(v/w)) },
                    TokenType::Star => { Literal::new(Value::Float(v*w)) },
                    TokenType::Minus => { Literal::new(Value::Float(v-w)) },
                    TokenType::Plus => { Literal::new(Value::Float(v+w)) },
                    _ => {panic!("binary error: unexpected token type: {}", value_type)},
                }
            },
           (Value::String(v), Value::Integer(w)) => { 
                match token {
                    TokenType::Minus => { 
                        if w > v.len() as i64 {
                            return Literal::new(Value::String(String::new()));
                        }
                        let i = v.chars().take(v.chars().count() - w as usize).collect();
                        Literal::new(Value::String(i))
                    },
                    TokenType::Plus => { 
                        Literal::new(Value::String(format!("{}{}", v, w)))
                    },
                    _ => {panic!("Cannot {} string and integer", value_type)}
                }
            },
           (Value::Integer(v), Value::String(w)) => {
                match token {
                    TokenType::Minus => { 
                        Literal::new(Value::Integer(v - w.len() as i64))
                    },
                    TokenType::Plus => { 
                        Literal::new(Value::String(format!("{}{}", v, w)))
                    },
                    _ => {panic!("Cannot {} integer and string", value_type)}
                } 
            },
            (Value::String(v), Value::String(w)) => { 
                match token {
                    TokenType::Plus => { 
                        Literal::new(Value::String(format!("{}{}", v, w)))
                    },
                    _ => {panic!("Cannot {} string and string", value_type)}
                }
            },
            (Value::Float(v), Value::String(w)) => {
                match token {
                    TokenType::Plus => { 
                        Literal::new(Value::String(format!("{}{}", v, w)))
                    },
                    _ => {panic!("Cannot {} float and string", value_type)}
                }
            },
            (Value::String(w), Value::Float(v)) => {
                match token {
                    TokenType::Plus => { 
                        Literal::new(Value::String(format!("{}{}", v, w)))
                    },
                    _ => {panic!("Cannot {} string and float", value_type)}
                }
            },
            (Value::Boolean(v), Value::String(w)) => {
                match token {
                    TokenType::Plus => { 
                        Literal::new(Value::String(format!("{}{}", v, w)))
                    },
                    _ => {panic!("Cannot {} boolean and string", value_type)}
                }
            },
            (Value::String(v), Value::Boolean(w)) => {
                match token {
                    TokenType::Plus => { 
                        Literal::new(Value::String(format!("{}{}", v, w)))
                    },
                    _ => {panic!("Cannot {} string and boolean", value_type)}
                }
            },
           (Value::Integer(_)|Value::Float(_), Value::Boolean(_)) => {panic!("Cannot {} boolean and number", value_type)},
           (Value::Boolean(_), Value::Integer(_)|Value::Float(_)) => {panic!("Cannot {} number and boolean", value_type)},
           (Value::Integer(_)|Value::Float(_), Value::Null) => {panic!("Cannot {} null and number", value_type)},
           (Value::Null, Value::Integer(_)|Value::Float(_)) => {panic!("Cannot {} number and null", value_type)},
           (Value::Integer(_), Value::Float(_)) => {panic!("Cannot {} float and integer", value_type)},
           (Value::Float(_), Value::Integer(_)) => {panic!("Cannot {} integer and float", value_type)},
            _ => {panic!("binary error: unexpected values")},
        }
    } 
    fn interpret(&self, expression: Expr) -> 
        // Result<Expr, RuntimeError> //TODO remove panics and use Results 
        Expr 
        { 
        let value = self.evaluate(expression);
        println!("Interpreted: {:?}", value);
        return value;
    }
}

impl Visitor<Expr> for Interpreter {
    fn visit_binary(&self, binary: &Binary) -> Expr {
        let left: Expr = self.evaluate(*binary.left.clone());
        let right: Expr = self.evaluate(*binary.right.clone());
        match binary.operator.type_of {
            TokenType::Greater => {
                return Interpreter::check_number_operands(binary.operator.type_of.clone(),left, right);            
            },
            TokenType::GreaterEqual => {
                return Interpreter::check_number_operands(binary.operator.type_of.clone(),left, right);            
            },
            TokenType::Less => {
                return Interpreter::check_number_operands(binary.operator.type_of.clone(),left, right);            
            },
            TokenType::LessEqual => {
                return Interpreter::check_number_operands(binary.operator.type_of.clone(),left, right);            
            },
            TokenType::BangEqual => {
                return Literal::new(Value::Boolean(!Interpreter::is_equal(Interpreter::match_binaries(left, right))));
            },
            TokenType::EqualEqual => {
                return Literal::new(Value::Boolean(Interpreter::is_equal(Interpreter::match_binaries(left, right)))); 
            },
            TokenType::Minus => {
                return Interpreter::check_number_operands(binary.operator.type_of.clone(),left, right);
            },
            TokenType::Slash => {
                return Interpreter::check_number_operands(binary.operator.type_of.clone(),left, right);
            },
            TokenType::Star => {
                return Interpreter::check_number_operands(binary.operator.type_of.clone(),left, right);
            },
            TokenType::Plus => {
                return Interpreter::check_number_operands(binary.operator.type_of.clone(),left, right);
            },
            _ => {return right}
        }
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
            TokenType::Bang => {
                match right {
                    Expr::Literal(r) => {
                        match r.value {
                            Value::Null => { 
                                return Literal::new(Value::Boolean(!Interpreter::is_truthy(Value::Null)))
                            },
                            _ => {
                                return Literal::new(Value::Boolean(!Interpreter::is_truthy(r.value)))
                            }
                        }
                    },
                    _ => { panic!("bang-right panic") },
                } 

            },
            TokenType::Minus => {
                match right {
                    Expr::Literal(r) => {
                        match r.value {
                            Value::Integer(i) => { 
                                return Literal::new(Value::Integer(-i))
                            },
                            Value::Float(f) => {
                                return Literal::new(Value::Float(-f))
                            },
                            _ => {panic!("value is string or boolean panic")}
                        }
                    },
                    _ => { panic!("minus-right panic") },
                } 
            },
            _ => {panic!("unary panic")}
        }
    }
}
