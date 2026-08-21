use std::fs;
use std::io::Read;
use std::process::ExitCode;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenKind {
    EOF,
    Print,
    Put,
    While,
    If,
    Else,

    Identifier,
    Integer,
    String,
    Character,

    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Negate,
    Not,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,

    RightParen,
    LeftParen,
    RightBrace,
    LeftBrace,
    Comma,
    Semicolon
    
}

#[derive(Debug)]
struct Token {
    token_kind: TokenKind,
    value: Option<String>,
}

fn peek(idx: usize, n: usize, content: &Vec<u8>) -> Option<String> {
    if idx + n > content.len() {
        None
    } else {
        let x = String::from_utf8(content[idx..n+idx].to_vec()).expect("failed to parse into string");
        Some(x)
    }
}

fn consume(idx: usize, n: usize) -> usize {
    idx + n
}

fn keyword_token(token_kind: TokenKind) -> Option<Token> {
    let t = Token {
        token_kind: token_kind,
        value: None
    };
    return Some(t);
}

fn find_keyword(idx: usize, content: &Vec<u8>) -> Option<Token>{
    let keywords = vec!["print", "while"];
    
    if let Some(content) = peek(idx, 5, content) {
        match content.as_str() {
            "print" => keyword_token(TokenKind::Print),
            "while" => keyword_token(TokenKind::While),
            _ => None
        }
    } else if let Some(content) = peek(idx, 4, content) { // this is OK - we need not return if we dont find a match or return and call the function on 4s etc/
        match content.as_str() {
            "else" => keyword_token(TokenKind::Else),
            _ => None
        }
    } 
    else {
        None
    }
}

fn main() -> ExitCode {
    let file_path = "tests/basic_hello_world.txt";
    let content = fs::read_to_string(file_path).expect("failed to read file");
    if !content.is_ascii() {
        eprintln!("ERROR: file content not ascii");
        return ExitCode::FAILURE;
    }

    let chars = content.into_bytes();
    let mut pos = 0;
    while pos < chars.len() {
        if let Some(p) = find_keyword(pos, &chars){
            println!("{:?}", p);
            pos = consume(pos, 5);
        }
        
        pos +=1;
    }
    
    ExitCode::SUCCESS
}

