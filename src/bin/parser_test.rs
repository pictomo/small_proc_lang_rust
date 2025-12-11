// cargo run --bin parser_test

use small_proc_lang_rust::parser::parser;

fn main() {
    let input = "let bool x;";
    let input =
        "let bool x; x = true; while x { if not x { x = false; } else { x = (true and false) }; };";
    // let input = "while true {"; // Error case

    match parser(input) {
        Ok(_) => println!("Result: OK"),
        Err(e) => println!("Result: Error: {}", e),
    }
}
