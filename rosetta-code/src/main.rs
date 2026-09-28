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
    Semicolon,
}

#[derive(Debug)]
struct Token {
    token_kind: TokenKind,
    value: Option<String>,
}

struct TokenConsume {
    token: Token,
    size: usize,
}

fn peek(idx: usize, n: usize, content: &[u8]) -> Option<String> {
    if idx + n > content.len() {
        None
    } else {
        let x =
            String::from_utf8(content[idx..n + idx].to_vec()).expect("failed to parse into string");
        Some(x)
    }
}

fn consume(idx: usize, n: usize) -> usize {
    idx + n
}

fn consume_whitespace(idx: usize, content: &Vec<u8>) -> bool {
    let content = peek(idx, 1, content).unwrap_or("".to_string());
    if matches!(content.as_str(), " " | "\n" | "\t" | "\r") {
        true
    } else {
        false
    }
}

fn track_comment(idx: usize, content: &[u8]) -> Option<usize> {
    let mut m_idx = idx;
    
    while let Some(content) = peek(m_idx, 2, content) {
        match content.as_str() {
            "*/" => return Some(m_idx - idx + 2),
            _ => m_idx += 1,
        }
    }
    None
}

fn consume_comment(idx: usize, content: &[u8]) -> Option<usize> {
    match peek(idx, 2, content)?.as_str() {
        "/*" => track_comment(idx, content),
        _ => None,
    }
}

fn keyword_token(token_kind: TokenKind, size: usize) -> Option<TokenConsume> {
    let t = Token {
        token_kind: token_kind,
        value: None,
    };
    Some(TokenConsume {
        token: t,
        size: size,
    })
}

fn keyword_symbol(token_kind: TokenKind) -> Option<Token> {
    let t = Token {
        token_kind: token_kind,
        value: None,
    };
    Some(t)
}

fn find_keyword(idx: usize, content: &[u8]) -> Option<TokenConsume> {
    if let Some(content) = peek(idx, 5, content) {
        match content.as_str() {
            "print" => keyword_token(TokenKind::Print, 5),
            "while" => keyword_token(TokenKind::While, 5),
            _ => None,
        }
    } else if let Some(content) = peek(idx, 4, content) {
        // this is OK - we need not return if we dont find a match or return and call the function on 4s etc/
        match content.as_str() {
            "else" => keyword_token(TokenKind::Else, 4),
            _ => None,
        }
    } else {
        None
    }
}

fn find_symbol(idx: usize, content: &[u8]) -> Option<Token> {
    let symbol_size = 1;
    if let Some(content) = peek(idx, symbol_size, content) {
        match content.as_str() {
            "(" => keyword_symbol(TokenKind::LeftParen),
            ")" => keyword_symbol(TokenKind::RightParen),
            "{" => keyword_symbol(TokenKind::LeftBrace),
            "}" => keyword_symbol(TokenKind::RightBrace),
            "+" => keyword_symbol(TokenKind::Add),
            "-" => keyword_symbol(TokenKind::Subtract),
            "*" => keyword_symbol(TokenKind::Multiply),
            "/" => keyword_symbol(TokenKind::Divide),
            "%" => keyword_symbol(TokenKind::Mod),
            "," => keyword_symbol(TokenKind::Comma),
            ";" => keyword_symbol(TokenKind::Semicolon),
            "=" => keyword_symbol(TokenKind::Assign),
            "<" => keyword_symbol(TokenKind::Less),
            ">" => keyword_symbol(TokenKind::Greater),
            "!" => keyword_symbol(TokenKind::Not),
            _ => None,
        }
    } else {
        None
    }
}

fn get_string_literal(idx: usize, content: &[u8]) -> Option<TokenConsume> {
    // we assume that we are given the next index
    let mut end_idx = idx + 1;
    while let Some(c) = peek(end_idx, 1, content) {
        if c == "\"" {
            let s: String = str::from_utf8(&content[idx..end_idx]).unwrap().to_string();
            let t = Token {
                token_kind: TokenKind::String,
                value: Some(s),
            };
            return Some(TokenConsume {
                token: t,
                size: end_idx - idx + 1,
            });
        }
        end_idx += 1;
    }
    unreachable!("could not find end of string")
}

fn find_literal(idx: usize, content_str: &[u8]) -> Option<TokenConsume> {
    match peek(idx, 1, content_str)?.as_str() {
        "\"" => get_string_literal(idx, content_str),
        _ => None,
    }
}

fn main() -> ExitCode {
    let file_path = "tests/complex_print.txt";
    let content = fs::read_to_string(file_path).expect("failed to read file");
    if !content.is_ascii() {
        eprintln!("ERROR: file content not ascii");
        return ExitCode::FAILURE;
    }
    println!("Content: {content}\n");

    let chars = content.into_bytes();
    let mut pos = 0;
    while pos < chars.len() {
        // todo: consume comments
        if consume_whitespace(pos, &chars) {
            pos += 1;
        } else if let Some(whitespace) = consume_comment(pos, &chars) {
            pos += whitespace;
        } else if let Some(keyword) = find_keyword(pos, &chars) {
            let t = keyword.token;
            println!("token:   {t:?}; position: {pos}");
            pos = consume(pos, keyword.size);
        } else if let Some(symbol) = find_symbol(pos, &chars) {
            println!("symbol:  {symbol:?}; position: {pos}");
            pos = consume(pos, 1);
        } else if let Some(literal) = find_literal(pos, &chars) {
            let t = literal.token;
            println!("literal: {t:?}; position: {pos}");
            pos = consume(pos, literal.size);
        } else {
            unreachable!("could not tokenize");
        }
    }
    ExitCode::SUCCESS
}
