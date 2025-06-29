use std::io;
use std::path::Path;
use std::process;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};


pub struct Lox {
    had_error: AtomicBool,
}

impl Lox {
    
    fn main(&self, args: Vec<String>) {
        if args.len() > 1 {
            println!("Usage: jlox [script]");
            process::exit(64); 
        } else if args.len() == 1 {
            let file_path = args.get(0).expect("MAIN: Invalid arg provided");
            self.run_file(file_path.as_str());
        } else {
            self.run_prompt();
    }
}
    
    fn run_file(&self, path: &str) {
        let file = Path::new(&path);
        let contents = fs::read_to_string(file).expect("RUN_FILE: Could not read file");
        self.run(&contents);

        if self.had_error.load(Ordering::SeqCst) {
            process::exit(65)
        };
    }

    fn run_prompt(&self) {
        let mut line: String = String::new();
        loop {
            println!("> ");
            io::stdin().read_line(&mut line).expect("RUN_PROMPT: could not read line");
            if line.trim().is_empty() {break};
            self.run(&line);
            line.clear();
            self.had_error.store(false, Ordering::SeqCst)
        }
    }
    
    fn run(&self, source: &String) {
       let scanner: Scanner = Scanner::new(source);
       let tokens: Vec<Token> = scanner.scan_tokens();

       for t in tokens {
            println!("{:?}", t);
       }
    }

    fn error(&self, line: u32, message: String) {
        self.report(line, "", message.as_str());
    }

    fn report(&self, line: u32, location: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, location, message);
        self.had_error.store(true, Ordering::SeqCst)
    }

}


pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {

    fn new(source: &String) -> Self {
        Self {
            source: source.clone(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        loop {
            self.start = self.current;
            self.scan_token();
            if !self.is_at_end() {break}
        }
        self.tokens.push(
            Token::new(TokenType::Eof, "".to_string(), "".to_string(), self.line)
        );
        self.tokens
    }

    fn scan_token(&self) {
        let c = self.advance();
        match c {
            '(' => { self.add_token(TokenType::LeftParen); },
            ')' => { self.add_token(TokenType::RightParen); },
            '{' => { self.add_token(TokenType::LeftBrace); },
            '}' => { self.add_token(TokenType::RightBrace); },
            ',' => { self.add_token(TokenType::Comma)},
            '.' => { self.add_token(TokenType::Dot); },
            '-' => { self.add_token(TokenType::Minus); },
            '+' => { self.add_token(TokenType::Plus); },
            ';' => { self.add_token(TokenType::Semicolon); },
            '*' => { self.add_token(TokenType::Star); },
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().expect("IS_AT_END: could not convert usize to i32")
    }

    fn advance(&self) {
        // TODO wip, falling asleep
        self.source.char
    }


}

#[derive(Debug)]
enum TokenType {

LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual, 

Identifier, String, Number,

And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Var, While,

Eof
}

#[derive(Debug)]
pub struct Token {
    type_of: TokenType,
    lexeme: String,
    literal: String,
    line: u32,
}

impl Token {

    fn new(type_of: TokenType, lexeme: String, literal: String, line: u32) -> Token {
        Token {
            type_of,
            lexeme,
            literal,
            line
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {}", self.type_of, self.lexeme, self.literal)
    }

}

// use std::env;
// const args: Vec<String> = env::args().collect();
// let lox: Lox = Lox{};
//
// lox::main(lox, args);
