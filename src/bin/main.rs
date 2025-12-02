use small_proc_lang_rust::lexer::lexer;

fn main() {
    let mut input = "123 + 456";
    let mut input = "123 + 456\n let x = true and false\n x= x + -1";
    // let mut input = "invalid%&#(";

    input = input.trim();

    while !input.is_empty() {
        match lexer(input) {
            Ok(((token_type, matched_str), remaining_str)) => {
                println!("Token: {:?}, Matched: '{}'", token_type, matched_str);
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
}
