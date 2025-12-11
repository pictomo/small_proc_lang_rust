use regex::Regex;
use std::sync::LazyLock;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    EOF,
    Number,
    Plus,
    Minus,
    Mult,
    Div,
    Lparen,
    Rparen,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    True,
    False,
    Not,
    And,
    Or,
    Assign,
    Let,
    Num,
    Bool,
    Lblock,
    Rblock,
    Else,
    If,
    While,
    Semicolon,
    Identifier,
}
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
}

fn genRegex(pattern: &str) -> Regex {
    Regex::new(&format!("^{}", pattern)).unwrap()
}

pub static TOKEN_DICT: LazyLock<Vec<(TokenType, Regex)>> = LazyLock::new(|| {
    vec![
        (TokenType::EOF, genRegex(r"$")),
        (TokenType::Number, genRegex(r"([0-9]+)(.[0-9]+)?")),
        (TokenType::Plus, genRegex(r"\+")),
        (TokenType::Minus, genRegex(r"-")),
        (TokenType::Mult, genRegex(r"\*")),
        (TokenType::Div, genRegex(r"/")),
        (TokenType::Lparen, genRegex(r"\(")),
        (TokenType::Rparen, genRegex(r"\)")),
        (TokenType::Eq, genRegex(r"==")),
        (TokenType::Neq, genRegex(r"!=")),
        (TokenType::Lt, genRegex(r"<")),
        (TokenType::Gt, genRegex(r">")),
        (TokenType::Leq, genRegex(r"<=")),
        (TokenType::Geq, genRegex(r">=")),
        (TokenType::True, genRegex(r"true")),
        (TokenType::False, genRegex(r"false")),
        (TokenType::Not, genRegex(r"not")),
        (TokenType::And, genRegex(r"and")),
        (TokenType::Or, genRegex(r"or")),
        (TokenType::Assign, genRegex(r"=")),
        (TokenType::Let, genRegex(r"let")),
        (TokenType::Num, genRegex(r"num")),
        (TokenType::Bool, genRegex(r"bool")),
        (TokenType::Lblock, genRegex(r"\{")),
        (TokenType::Rblock, genRegex(r"\}")),
        (TokenType::Else, genRegex(r"else")),
        (TokenType::If, genRegex(r"if")),
        (TokenType::While, genRegex(r"while")),
        (TokenType::Identifier, genRegex(r"[a-zA-Z_][a-zA-Z0-9_]*")),
        (TokenType::Semicolon, genRegex(r";")),
    ]
});
