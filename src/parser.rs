use crate::lexer::lexer;
use crate::token::{Token, TokenType};

pub fn parser(input: &str) -> Result<(), String> {
    let input = input.trim();

    let (first_token, remain_str): (Token, &str) = lexer(input)?;
    all(first_token, remain_str).unwrap();
    Ok(())
}

fn check_token(token: Token, expected_type: TokenType) -> Result<(), String> {
    if token.token_type == expected_type {
        return Ok(());
    }
    Err(format!("parser: expected '{:?}'", expected_type))
}

fn all(first_token: Token, remain_str: &str) -> Result<(Token, &str), String> {
    let (token, remain_str) = match first_token.token_type {
        TokenType::While => while_stmt(first_token, remain_str)?,
        TokenType::If => if_stmt(first_token, remain_str)?,
        TokenType::Let => declare_stmt(first_token, remain_str)?,
        TokenType::Identifier => assign_stmt(first_token, remain_str)?,

        TokenType::Rblock => return Ok((first_token, remain_str)),
        TokenType::EOF => return Ok((first_token, "")),

        _ => return Err("parser: expected statement".to_string()),
    };

    check_token(token, TokenType::Semicolon)?;
    let (token, remain_str) = lexer(remain_str)?;

    all(token, remain_str)
}

fn while_stmt(first_token: Token, remain_str: &str) -> Result<(Token, &str), String> {
    check_token(first_token, TokenType::While)?;
    let (token, remain_str) = lexer(remain_str)?;

    let (token, remain_str) = boolean_expr(token, remain_str)?;

    check_token(token, TokenType::Lblock)?;
    let (token, remain_str) = lexer(remain_str)?;

    let (token, remain_str) = all(token, remain_str)?;

    check_token(token, TokenType::Rblock)?;
    lexer(remain_str)
}

fn if_stmt(first_token: Token, remain_str: &str) -> Result<(Token, &str), String> {
    check_token(first_token, TokenType::If)?;
    let (token, remain_str) = lexer(remain_str)?;

    let (token, remain_str) = boolean_expr(token, remain_str)?;

    check_token(token, TokenType::Lblock)?;
    let (token, remain_str) = lexer(remain_str)?;

    let (token, remain_str) = all(token, remain_str)?;

    check_token(token, TokenType::Rblock)?;
    let (token, remain_str) = lexer(remain_str)?;

    match token.token_type {
        TokenType::Else => else_stmt(token, remain_str),
        _ => Ok((token, remain_str)),
    }
}

fn else_stmt(first_token: Token, remain_str: &str) -> Result<(Token, &str), String> {
    check_token(first_token, TokenType::Else)?;
    let (token, remain_str) = lexer(remain_str)?;

    match token.token_type {
        TokenType::If => if_stmt(token, remain_str),
        TokenType::Lblock => {
            let (token, remain_str) = lexer(remain_str)?;

            let (token, remain_str) = all(token, remain_str)?;

            check_token(token, TokenType::Rblock)?;
            lexer(remain_str)
        }
        _ => Err("parser: expected 'if' or '{'".to_string()),
    }
}

fn declare_stmt(first_token: Token, remain_str: &str) -> Result<(Token, &str), String> {
    check_token(first_token, TokenType::Let)?;
    let (token, remain_str) = lexer(remain_str)?;

    let (token, remain_str) = match token.token_type {
        TokenType::Bool | TokenType::Num => lexer(remain_str)?,
        _ => return Err("parser: expected 'bool' or 'num'".to_string()),
    };

    check_token(token, TokenType::Identifier)?;
    lexer(remain_str)
}

fn assign_stmt(first_token: Token, remain_str: &str) -> Result<(Token, &str), String> {
    check_token(first_token, TokenType::Identifier)?;
    let (token, remain_str) = lexer(remain_str)?;

    check_token(token, TokenType::Assign)?;
    let (token, remain_str) = lexer(remain_str)?;

    boolean_expr(token, remain_str)
}

fn boolean_expr(first_token: Token, remain_str: &str) -> Result<(Token, &str), String> {
    match first_token.token_type {
        TokenType::Not => boolean_not(first_token, remain_str),
        _ => {
            let (token, remain_str) = boolean(first_token, remain_str)?;

            match token.token_type {
                TokenType::And | TokenType::Or => boolean_calc(token, remain_str),
                _ => Ok((token, remain_str)),
            }
        }
    }
}

fn boolean_not(first_token: Token, remain_str: &str) -> Result<(Token, &str), String> {
    check_token(first_token, TokenType::Not)?;
    let (token, remain_str) = lexer(remain_str)?;

    boolean_expr(token, remain_str)
}

fn boolean_calc(first_token: Token, remain_str: &str) -> Result<(Token, &str), String> {
    match first_token.token_type {
        TokenType::And | TokenType::Or => {}
        _ => return Err("parser: expected 'and' or 'or'".to_string()),
    }

    let (token, remain_str) = lexer(remain_str)?;
    boolean_expr(token, remain_str)
}

fn boolean(first_token: Token, remain_str: &str) -> Result<(Token, &str), String> {
    match first_token.token_type {
        TokenType::True => lexer(remain_str),
        TokenType::False => lexer(remain_str),
        TokenType::Lparen => {
            let (token, remain_str) = lexer(remain_str)?;

            let (token, remain_str) = boolean_expr(token, remain_str)?;

            check_token(token, TokenType::Rparen)?;
            lexer(remain_str)
        }
        TokenType::Identifier => lexer(remain_str),
        _ => return Err("parser: expected boolean".to_string()),
    }
}
