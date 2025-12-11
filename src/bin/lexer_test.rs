// cargo run --bin lexer_test

use small_proc_lang_rust::lexer::lexer;

fn main() {
    let mut input = "123 + 456;";
    let mut input = "123 + 456;\n let x = true and false;\n x= x + -1;";
    // let mut input = "invalid%&#(";

    input = input.trim();

    while !input.is_empty() {
        match lexer(input) {
            Ok((token, remaining_str)) => {
                println!(
                    "Token: {:?}, Matched: '{}'",
                    token.token_type, token.token_value
                );
                input = remaining_str;
            }
            Err(err) => {
                print!(
                    "Error: {}: {}",
                    err,
                    input.chars().take(3).collect::<String>()
                );
                if input.len() > 3 {
                    print!("...");
                }
                println!();
                break;
            }
        }
    }

    // it does not catch EOF
}
