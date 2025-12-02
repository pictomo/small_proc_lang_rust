use regex::Regex;
use std::sync::LazyLock;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Mult,
    Div,
    LParen,
    RParen,
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
    Xor,
    Assign,
    Identifier,
    Break,
}

fn genRegex(pattern: &str) -> Regex {
    Regex::new(&format!("^{}", pattern)).unwrap()
}

pub static TOKEN_DICT: LazyLock<Vec<(TokenType, Regex)>> = LazyLock::new(|| {
    vec![
        (TokenType::Number, genRegex(r"([0-9]+)(.[0-9]+)?")),
        (TokenType::Plus, genRegex(r"\+")),
        (TokenType::Minus, genRegex(r"-")),
        (TokenType::Mult, genRegex(r"\*")),
        (TokenType::Div, genRegex(r"/")),
        (TokenType::LParen, genRegex(r"\(")),
        (TokenType::RParen, genRegex(r"\)")),
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
        (TokenType::Xor, genRegex(r"xor")),
        (TokenType::Assign, genRegex(r"=")),
        (TokenType::Identifier, genRegex(r"[a-zA-Z_][a-zA-Z0-9_]*")),
        (TokenType::Break, genRegex(r"\n")),
    ]
});
