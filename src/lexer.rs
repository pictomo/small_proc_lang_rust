use crate::token::TOKEN_DICT;
use crate::token::TokenType;

pub fn lexer(input: &str) -> Result<((TokenType, &str), &str), &str> {
    let input = input.trim_start();
    if (input.is_empty()) {
        return Err("lexer: empty input");
    }

    for (token_type, regex) in TOKEN_DICT.iter() {
        if let Some(mat) = regex.find(input) {
            let matched_str = mat.as_str();
            let remaining_str = &input[matched_str.len()..];
            return Ok(((*token_type, matched_str), remaining_str));
        }
    }

    return Err("lexer: no token matched");
}
