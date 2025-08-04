use std::env;

use rulox::{ast::{Binary, Expr, Grouping, Literal, Unary, Value}, Lox, Token, TokenType };

fn main() {
    
    let expr: Expr = Binary::new(
        Box::new(
            Unary::new(
                Token::new(TokenType::Minus, "-", Value::String("none".to_string()), 1),
                Box::new(Literal::new(Value::Integer(123))),
            )
        ),
        Token::new(TokenType::Star, "*", Value::String("none".to_string()), 1),
        Box::new(
            Grouping::new(Box::new(Literal::new(Value::Float(12.5))))
        ),
    );

    println!("expression: {:?}", expr);
    
    // TODO: uncomment this later
    // let args: Vec<String> = env::args().collect();
    // let lox = Lox::new();
    // lox.main(args);

}
