use crate::{ast::{Binary, Expr, Grouping, Literal, Unary, Value, Visitor}, lox::{Token, TokenType}, runtime_error::RuntimeError};

#[derive(Clone)]
pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&self, expr: Expr) -> Result<Expr, RuntimeError> {
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

    fn check_number_operands(token: Token, left: Expr, right: Expr) -> Result<Expr, RuntimeError> {
        let value_type: &str = match token.type_of {
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
                match token.type_of {
                    TokenType::Greater => { Ok(Literal::new(Value::Boolean(v>w))) },
                    TokenType::GreaterEqual => { Ok(Literal::new(Value::Boolean(v>=w))) },
                    TokenType::Less => { Ok(Literal::new(Value::Boolean(v<w))) },
                    TokenType::LessEqual => { Ok(Literal::new(Value::Boolean(v<=w))) },
                    TokenType::Slash => { Ok(Literal::new(Value::Integer(v/w))) },
                    TokenType::Star => { Ok(Literal::new(Value::Integer(v*w))) },
                    TokenType::Minus => { Ok(Literal::new(Value::Integer(v-w))) },
                    TokenType::Plus => { Ok(Literal::new(Value::Integer(v+w))) },
                    _ => {Err(RuntimeError::new(token, format!("binary error: unexpected token type: {}", value_type)))},
                }

            },
            (Value::Float(v), Value::Float(w)) => { 
                match token.type_of {
                    TokenType::Greater => { Ok(Literal::new(Value::Boolean(v>w))) },
                    TokenType::GreaterEqual => { Ok(Literal::new(Value::Boolean(v>=w))) },
                    TokenType::Less => { Ok(Literal::new(Value::Boolean(v<w))) },
                    TokenType::LessEqual => { Ok(Literal::new(Value::Boolean(v<=w))) },
                    TokenType::Slash => { Ok(Literal::new(Value::Float(v/w))) },
                    TokenType::Star => { Ok(Literal::new(Value::Float(v*w))) },
                    TokenType::Minus => { Ok(Literal::new(Value::Float(v-w))) },
                    TokenType::Plus => { Ok(Literal::new(Value::Float(v+w))) },
                    _ => {Err(RuntimeError::new(token, format!("binary error: unexpected token type: {}", value_type)))},
                }
            },
           (Value::String(v), Value::Integer(w)) => { 
                match token.type_of {
                    TokenType::Minus => { 
                        if w > v.len() as i64 { // if string is too small, remove all chars
                            return Ok(Literal::new(Value::String(String::new())));
                        }
                        let i = v.chars().take(v.chars().count() - w as usize).collect();
                        Ok(Literal::new(Value::String(i)))
                    },
                    TokenType::Plus => { 
                        Ok(Literal::new(Value::String(format!("{}{}", v, w))))
                    },
                    _ => {Err(RuntimeError::new(token, format!("Cannot {} string and integer", value_type)))}
                }
            },
           (Value::Integer(v), Value::String(w)) => {
                match token.type_of {
                    TokenType::Minus => { 
                        Ok(Literal::new(Value::Integer(v - w.len() as i64)))
                    },
                    TokenType::Plus => { 
                        Ok(Literal::new(Value::String(format!("{}{}", v, w))))
                    },
                    _ => {Err(RuntimeError::new(token, format!("Cannot {} integer and string", value_type)))}
                } 
            },
           (Value::String(v), Value::String(w)) => { 
                match token.type_of {
                    TokenType::Plus => { 
                       Ok(Literal::new(Value::String(format!("{}{}", v, w))))
                    },
                    _ => {Err(RuntimeError::new(token, format!("Cannot {} string and string", value_type)))}
                }
            },
            (Value::Float(v), Value::String(w)) => {
                match token.type_of {
                    TokenType::Plus => { 
                        Ok(Literal::new(Value::String(format!("{}{}", v, w))))
                    },
                    _ => {Err(RuntimeError::new(token, format!("Cannot {} float and string", value_type)))}
                }
            },
            (Value::String(w), Value::Float(v)) => {
                match token.type_of {
                    TokenType::Plus => { 
                        Ok(Literal::new(Value::String(format!("{}{}", v, w))))
                    },
                    _ => {Err(RuntimeError::new(token, format!("Cannot {} string and float", value_type)))}
                }
            },
            (Value::Boolean(v), Value::String(w)) => {
                match token.type_of {
                    TokenType::Plus => { 
                        Ok(Literal::new(Value::String(format!("{}{}", v, w))))
                    },
                    _ => {Err(RuntimeError::new(token, format!("Cannot {} boolean and string", value_type)))}
                }
            },
            (Value::String(v), Value::Boolean(w)) => {
                match token.type_of {
                    TokenType::Plus => { 
                        Ok(Literal::new(Value::String(format!("{}{}", v, w))))
                    },
                    _ => {Err(RuntimeError::new(token, format!("Cannot {} string and boolean", value_type)))}
                }
            },
           (Value::Integer(_)|Value::Float(_), Value::Boolean(_)) => {
           Err(RuntimeError::new(token, format!("Cannot {} boolean and number", value_type)))
           },
           (Value::Boolean(_), Value::Integer(_)|Value::Float(_)) => {
           Err(RuntimeError::new(token, format!("Cannot {} number and boolean", value_type)))
           },
           (Value::Integer(_)|Value::Float(_), Value::Null) => {
           Err(RuntimeError::new(token, format!("Cannot {} null and number", value_type)))
           },
           (Value::Null, Value::Integer(_)|Value::Float(_)) => {
           Err(RuntimeError::new(token, format!("Cannot {} number and null", value_type)))
           },
           (Value::Integer(_), Value::Float(_)) => {
           Err(RuntimeError::new(token, format!("Cannot {} float and integer", value_type)))
           },
           (Value::Float(_), Value::Integer(_)) => {
           Err(RuntimeError::new(token, format!("Cannot {} integer and float", value_type)))
           },
            _ => {Err(RuntimeError::new(token, format!("binary error: unexpected values")))},
        }
    } 
    pub fn interpret(&self, expression: Expr) -> Result<Expr, RuntimeError> { 
        println!("raw interpret value: {:?}", expression);
        let value = self.evaluate(expression);
        println!("Interpreted: {:?}", value);
        return value;
    }
}

impl Visitor<Result<Expr, RuntimeError>> for Interpreter {
    fn visit_binary(&self, binary: &Binary) -> Result<Expr, RuntimeError> {
        let left: Result<Expr, RuntimeError> = self.evaluate(*binary.left.clone());
        let right: Result<Expr, RuntimeError> = self.evaluate(*binary.right.clone());
        match binary.operator.type_of {
            TokenType::Greater => {
                return Interpreter::check_number_operands(binary.operator.clone(),left?, right?);            
            },
            TokenType::GreaterEqual => {
                return Interpreter::check_number_operands(binary.operator.clone(),left?, right?);            
            },
            TokenType::Less => {
                return Interpreter::check_number_operands(binary.operator.clone(),left?, right?);            
            },
            TokenType::LessEqual => {
                return Interpreter::check_number_operands(binary.operator.clone(),left?, right?);            
            },
            TokenType::BangEqual => {
                return Ok(Literal::new(Value::Boolean(!Interpreter::is_equal(Interpreter::match_binaries(left?, right?)))));
            },
            TokenType::EqualEqual => {
                return Ok(Literal::new(Value::Boolean(Interpreter::is_equal(Interpreter::match_binaries(left?, right?))))); 
            },
            TokenType::Minus => {
                return Interpreter::check_number_operands(binary.operator.clone(),left?, right?);
            },
            TokenType::Slash => {
                return Interpreter::check_number_operands(binary.operator.clone(),left?, right?);
            },
            TokenType::Star => {
                return Interpreter::check_number_operands(binary.operator.clone(),left?, right?);
            },
            TokenType::Plus => {
                return Interpreter::check_number_operands(binary.operator.clone(),left?, right?);
            },
            _ => {return right}
        }
    }
    fn visit_grouping(&self, grouping: &Grouping) -> Result<Expr, RuntimeError> {
        self.evaluate(*grouping.expression.clone())
    }
    fn visit_literal(&self, literal: &Literal) -> Result<Expr, RuntimeError> {
        Ok(Expr::Literal(literal.clone())) // TODO this in theory should return the value but whatever
    }
    fn visit_unary(&self, unary: &Unary) -> Result<Expr, RuntimeError> { 
        let right: Result<Expr, RuntimeError> = self.evaluate(*unary.right.clone());
        match unary.operator.type_of {
            TokenType::Bang => {
                match right? {
                    Expr::Literal(r) => {
                        match r.value {
                            Value::Null => { 
                                return Ok(Literal::new(Value::Boolean(!Interpreter::is_truthy(Value::Null))))
                            },
                            _ => {
                                return Ok(Literal::new(Value::Boolean(!Interpreter::is_truthy(r.value))))
                            }
                        }
                    },
                    _ => { Err(RuntimeError::new(unary.operator.clone(), String::from("bang-right panic"))) },
                } 

            },
            TokenType::Minus => {
                match right? {
                    Expr::Literal(r) => {
                        match r.value {
                            Value::Integer(i) => { 
                                return Ok(Literal::new(Value::Integer(-i)))
                            },
                            Value::Float(f) => {
                                return Ok(Literal::new(Value::Float(-f)))
                            },
                            _ => { Err(RuntimeError::new(unary.operator.clone(), String::from("value is string or boolean panic"))) }
                        }
                    },
                    _ => { Err(RuntimeError::new(unary.operator.clone(), String::from("minus-right panic"))) },
                } 
            },
            _ => { Err(RuntimeError::new(unary.operator.clone(), String::from("unary panic"))) }
        }
    }
}
