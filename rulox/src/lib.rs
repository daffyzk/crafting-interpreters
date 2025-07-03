use std::collections::HashMap;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::fs;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

pub struct Lox {
    had_error: AtomicBool,
}

impl Clone for Lox {
    // TODO: idk about this one but it's just to make it run for now
    fn clone (&self) -> Self {
        Lox {
            had_error: AtomicBool::new(self.had_error.load(Ordering::Relaxed))
        }
    }
}


impl Lox {

    pub fn new() -> Self {
        Self {
            had_error: AtomicBool::new(false)
        }
    }
    
    pub fn main(&self, args: Vec<String>) {
        if args.len() > 2usize {
            println!("Usage: jlox [script]");
            process::exit(64); 
        } else if args.len() == 2usize {
            let file_path = args.get(1).expect("Main: Invalid arg provided");
            self.run_file(file_path.as_str());
        } else {
            self.run_prompt();
    }
}
    
    fn run_file(&self, path: &str) {
        let file = Path::new(&path);
        let contents = fs::read_to_string(file).expect("Run.File: Could not read file");
        self.run(&contents);

        if self.had_error.load(Ordering::SeqCst) {
            process::exit(65)
        };
    }

    fn run_prompt(&self) {
        let mut line: String = String::new();
        println!("DEBUG, line: {}", line);
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut line).expect("Run.Prompt: could not read line");
            if line.trim().is_empty() {break};
            self.run(&line);
            line.clear();
            self.had_error.store(false, Ordering::SeqCst)
        }
    }
    
    fn run(&self, source: &String) {
       let scanner: Scanner = Scanner::new(source, self);
       let tokens: Vec<Token> = scanner.scan_tokens();

       for t in tokens {
            println!("{:?}", t);
       }
    }

    fn error(&self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: u32, location: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, location, message);
        self.had_error.store(true, Ordering::SeqCst)
    }

}


pub struct Scanner {
    lox:        Lox,
    source:     String,
    tokens:     Arc<Mutex<Vec<Token>>>,
    start:      Arc<AtomicUsize>,
    current:    Arc<AtomicUsize>,
    line:       Arc<AtomicU32>,
    keywords:   Arc<RwLock<HashMap<String, TokenType>>>,
}

impl Scanner {

    fn new(source: &String, lox: &Lox) -> Self {
        let keywords = Arc::new(RwLock::new(HashMap::<String, TokenType>::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("fun".to_string(), TokenType::Fun),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ])));

        Self {
            source:  source.clone(),
            tokens:  Arc::new(Mutex::new(vec![])),
            start:   Arc::new(AtomicUsize::new(0)),
            current: Arc::new(AtomicUsize::new(0)),
            line:   Arc::new(AtomicU32::new(1)),
            lox:    lox.clone(),
            keywords
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        loop {
            let c = self.current.clone().load(Ordering::Relaxed);
            self.start.clone().store(c, Ordering::Relaxed);
            self.scan_token();
            if self.is_at_end() {break}
        }
        let line = self.line.clone().load(Ordering::Relaxed);
        self.tokens.clone().lock().unwrap().push(
            Token::new(TokenType::Eof, "", Value::String("".to_string()), line)
        );
        let vec = self.tokens.as_ref().lock().unwrap().clone();
        return vec;
    }

    fn scan_token(&self) {
        match self.advance() {
            Ok(c) => {
                match c {
                    '(' => { self.add_token(TokenType::LeftParen, None); },
                    ')' => { self.add_token(TokenType::RightParen, None); },
                    '{' => { self.add_token(TokenType::LeftBrace, None); },
                    '}' => { self.add_token(TokenType::RightBrace, None); },
                    ',' => { self.add_token(TokenType::Comma, None)},
                    '.' => { self.add_token(TokenType::Dot, None); },
                    '-' => { self.add_token(TokenType::Minus, None); },
                    '+' => { self.add_token(TokenType::Plus, None); },
                    ';' => { self.add_token(TokenType::Semicolon, None); },
                    '*' => { self.add_token(TokenType::Star, None); },
                    '!' => { 
                        if self.match_next('=') {
                            self.add_token(TokenType::BangEqual, None); 
                        } else {
                            self.add_token(TokenType::Bang, None); 
                        }
                    },
                    '=' => {
                        if self.match_next('=') {
                            self.add_token(TokenType::EqualEqual, None); 
                        } else {
                            self.add_token(TokenType::Equal, None); 
                        }
                    },
                    '<' => {
                        if self.match_next('=') {
                            self.add_token(TokenType::LessEqual, None); 
                        } else {
                            self.add_token(TokenType::Less, None); 
                        }
                    },
                    '>' => {
                        if self.match_next('=') {
                            self.add_token(TokenType::GreaterEqual, None); 
                        } else {
                            self.add_token(TokenType::Greater, None); 
                        }
                    },
                    '/' => {
                        if self.match_next('/') {
                            loop { 
                                if self.peek() == Ok('\n') || self.is_at_end() {break}
                                Self::handle_advance(self.advance(), "Comment");
                            };
                        } else {
                            self.add_token(TokenType::Slash, None); 
                        }
                    },
                    ' '  => {}, 
                    '\r' => {},
                    '\t' => {},
                    '\n' => { 
                        let new_line: u32 = self.line.clone().load(Ordering::Relaxed) + 1;
                        self.line.clone().store(new_line, Ordering::Relaxed);  
                    },
                    '"' => self.string(),

                    _c   => { 
                        if self.is_digit(c) {
                            self.number();
                        } else if self.is_alpha(c) {
                            self.identifier();
                        } else {
                            self.lox.error(self.line.clone().load(Ordering::Relaxed), "Unexpected character."); 
                        }
                    } 
                }
            },
            Err(e) => {println!("{}", e)}
        }
    }
    
    /// checks if current char exceeds the length of source
    fn is_at_end(&self) -> bool {
        self.current.clone().load(Ordering::Relaxed) >= self.source.len()
    }

    /// finds the next char and increments current
    fn advance(&self) -> Result<char, &str> {
        let c: usize = self.current.clone().load(Ordering::Relaxed) + 1usize;
        self.current.clone().store(c, Ordering::Relaxed);

        match self.source.chars().nth(c) {
            Some(char) => {Ok(char)},
            None       => {Err("Could not advance from current character.")},
        }
    } 

    /// finds the next char, if it matches expected, increments current and returns true
    fn match_next(&self, expected: char) -> bool {
        if self.is_at_end() {false} else {
            let c: usize = self.current.clone().load(Ordering::Relaxed) + 1usize;
            let char: char = self.source.chars().nth(c).unwrap();
            if char == expected {
                self.current.clone().store(c, Ordering::Relaxed);
                true
            } else {false}
        }  
    }
    
    /// take a lil peek
    fn peek(&self) -> Result<char, &str> {
        if self.is_at_end() { return Ok('\0') } else {
            let c: usize = self.current.clone().load(Ordering::Relaxed);
            match self.source.chars().nth(c) {
                Some(char) => { Ok(char) },
                None => {Err("Could not peek from current character.")},
            }
        }
    }

    fn add_token(&self, type_of: TokenType, literal: Option<Value>) {
        let line: u32  = self.line.clone().load(Ordering::Relaxed);
        let text: &str = &self.source_substring();

        match literal {
            Some(lit) => {
                self.tokens.clone().lock().unwrap()
                    .push(Token::new(type_of, text, lit, line))
            },
            None => {
                self.tokens.clone().lock().unwrap()
                    .push(Token::new(type_of, text, Value::String("".to_string()), line)) 
            },
        }
    }
    
    /// finds the value inside a string "", works for multilines
    fn string(&self) {
        loop { 
            if self.peek() == Ok('"') || self.is_at_end() {break}
            if self.peek() == Ok('\n') {
               let new_line = self.line.clone().load(Ordering::Relaxed);
               self.line.clone().store(new_line + 1, Ordering::Relaxed);
               break;
            }
            Self::handle_advance(self.advance(), "String");
        }

        if self.is_at_end() {
            self.lox.error(self.line.clone().load(Ordering::Relaxed), "Unterminated string.");
        }

        Self::handle_advance(self.advance(), "String.After");

        let current: usize = self.current.clone().load(Ordering::Relaxed) - 1usize;
        let start:   usize = self.start.clone().load(Ordering::Relaxed) + 1usize;
        let value: String = self.source[start..current].to_string();

        self.add_token(TokenType::String, Some(Value::String(value)));
    }
    
    /// pretty self explanatory
    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    /// tokenizing for number values
    fn number(&self) {
        loop {
            if !self.is_digit(self.peek().unwrap()) {break}
            Self::handle_advance(self.advance(), "Number");

            if self.peek().unwrap() == '.' &&
                self.is_digit(self.peek_next().unwrap()) {
                loop {
                    if !self.is_digit(self.peek().unwrap()) {break}
                    Self::handle_advance(self.advance(), "Number.Decimal");
                }
            }
           
        }
        let lit:    String = self.source_substring();
        let double:    f64 = lit.parse::<f64>().unwrap();
    
        self.add_token(TokenType::Number, Some(Value::Float(double)));
    }

    /// what if peek but twice
    fn peek_next(&self) -> Result<char, &str> {
        let new: usize = self.current.clone().load(Ordering::Relaxed) + 1usize;

        if new >= self.source.len() { return Ok('\0') } else {
            return match self.source.chars().nth(new) {
                Some(char) => { Ok(char) },
                None => {Err("Could not peek-next from current character.")},
            }
        }
    }

    fn identifier(&self) {
        loop {
            if !self.is_alpha_numeric(self.peek().unwrap()) {break}
            Self::handle_advance(self.advance(), "Identifier");
        }
        let text: String = self.source_substring();
        match self.keywords.clone().read().unwrap().get(&text) {
            Some(v) => {self.add_token(v.clone(), None);},
            None => self.add_token(TokenType::Identifier, None)
        }
    }
    
    /// is it alpha though???
    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
    
    /// error handling for advance when the value doesn't matter
    fn handle_advance(advance: Result<char, &str>, func: &str) {
        match advance {
            Ok(_) => {},
            Err(e) => println!("{}: Error advancing: {}", func, e),
        }
    }

    fn source_substring(&self) -> String {
        let current: usize = self.current.clone().load(Ordering::Relaxed);
        let start:   usize = self.start.clone().load(Ordering::Relaxed);
        self.source[start..current].to_string()
    }

}

#[derive(Debug, Clone)]
enum TokenType {

LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual, 

Identifier, String, Number,

And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Var, While,

Eof
}

#[derive(Debug, Clone)]
pub struct Token {
    type_of: TokenType,
    lexeme: String,
    literal: Value,
    line: u32,
}

impl Token {

    fn new(type_of: TokenType, lexeme: &str, literal: Value, line: u32) -> Token {
        Token {
            type_of,
            lexeme: lexeme.to_string(),
            literal,
            line
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {:?} {:?}", self.type_of, self.lexeme, self.literal)
    }

}

#[derive(Debug, Clone)]
enum Value {
    String(String),
    Float(f64),
}

// use std::env;
// const args: Vec<String> = env::args().collect();
// let lox: Lox = Lox{};
//
// lox::main(lox, args);
