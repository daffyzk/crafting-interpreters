use std::env;

use rulox::lox::Lox;

fn main() {
   
    let args: Vec<String> = env::args().collect();
    let lox = Lox::new();
    lox.main(args);

}
