use std::sync::LazyLock;
use regex::Regex;
use std::fs;
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

struct TokenMatch {
    regex: LazyLock<Regex>,
    parse: fn(&str) -> Option<Token>,
}


fn fixed_token(kind: TokenKind) -> Option<Token> {
    Some(Token { token_kind: kind, value: None })
}
fn parse_print(_content: &str) -> Option<Token> { fixed_token(TokenKind::Print) }
fn parse_put(_content: &str) -> Option<Token> { fixed_token(TokenKind::Put) }
fn parse_while(_content: &str) -> Option<Token> { fixed_token(TokenKind::While) }
fn parse_if(_content: &str) -> Option<Token> { fixed_token(TokenKind::If) }
fn parse_else(_content: &str) -> Option<Token> { fixed_token(TokenKind::Else) }

fn parse_left_paren(_content: &str) -> Option<Token> { fixed_token(TokenKind::LeftParen) }
fn parse_right_paren(_content: &str) -> Option<Token> { fixed_token(TokenKind::RightParen) }
fn parse_semicolon(_content: &str) -> Option<Token> { fixed_token(TokenKind::Semicolon) }
fn parse_comma(_content: &str) -> Option<Token> { fixed_token(TokenKind::Comma) }

fn parse_identifier(content: &str) -> Option<Token> {
    Some(Token { token_kind: TokenKind::Identifier, value: Some(content.to_string()) })
}
fn parse_integer(content: &str) -> Option<Token> {
    Some(Token { token_kind: TokenKind::Integer, value: Some(content.to_string()) })
}
fn parse_string(content: &str) -> Option<Token> {
    Some(Token { token_kind: TokenKind::String, value: Some(content.to_string()) })
}
fn parse_character(content: &str) -> Option<Token> {
    Some(Token { token_kind: TokenKind::Character, value: Some(content.to_string()) })
}

static TOKEN_MATCHES: LazyLock<[TokenMatch; 14]> = LazyLock::new(|| [
    TokenMatch { regex: LazyLock::new(|| Regex::new(r"\s+").unwrap()), parse: parse_print },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r"^print\b").unwrap()), parse: parse_print },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r"^put\b").unwrap()), parse: parse_put },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r"^while\b").unwrap()), parse: parse_while },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r"^if\b").unwrap()), parse: parse_if },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r"^else\b").unwrap()), parse: parse_else },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r"^\(").unwrap()), parse: parse_left_paren },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r"^\)").unwrap()), parse: parse_right_paren },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r"^;").unwrap()), parse: parse_semicolon },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r"^,").unwrap()), parse: parse_comma },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r#"[_a-zA-Z][_a-zA-Z0-9]*"#).unwrap()), parse: parse_identifier },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r#"[0-9]+"#).unwrap()), parse: parse_integer },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r#""[^"\n]*""#).unwrap()), parse: parse_string },
    TokenMatch { regex: LazyLock::new(|| Regex::new(r#"'([^'\n]|\\n|\\\\)'"#).unwrap()), parse: parse_character },
]);

fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < source.len() {
        let remaining = &source[pos..];

        let ws_len = remaining.len() - remaining.trim_start().len();
        if ws_len > 0 {
            pos += ws_len;
            continue;
        }
        let remaining = &source[pos..];

        let mut matched = false;
        for token_match in TOKEN_MATCHES.iter() {
            if let Some(m) = token_match.regex.find(remaining) {
                let text = &remaining[m.start()..m.end()];
                tokens.push((token_match.parse)(text));
                pos += m.end();
                matched = true;
                break;
            }
        }

        if !matched {
            panic!("unexpected character at position {pos}: {:?}", &remaining[..1]);
        }
    }
    
    tokens.push(Token { token_kind: TokenKind::EOF, value: None });
    tokens
}

fn main() -> ExitCode {
    let file_path = "tests/variables.txt";
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("ERROR: could not read file {file_path}: {err}");
            return ExitCode::FAILURE;
        }
    };
    let l = lex(&content);
    println!("{l:#?}");
    
    ExitCode::SUCCESS
}

