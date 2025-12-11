use crate::token::TOKEN_DICT;
use crate::token::{Token, TokenType};

pub fn lexer(input: &str) -> Result<(Token, &str), String> {
    let input = input.trim_start();

    for (token_type, regex) in TOKEN_DICT.iter() {
        if let Some(mat) = regex.find(input) {
            let matched_str = mat.as_str();
            let remaining_str = &input[matched_str.len()..];
            return Ok((
                Token {
                    token_type: *token_type,
                    token_value: matched_str.to_string(),
                },
                remaining_str,
            ));
        }
    }

    return Err("lexer: no token matched".to_string());
}
