use std::io;

pub fn capitalize(token: &str) -> String {
    // Function to capitalize a given string reference
    // If string reference is null, returns a new string
    // Note that the first letter does not appear twice
    // Because of the behavior of next(), which advance the iterator
    // So the first char is exhausted
    let mut char_1 = token.chars();

    match char_1.next() {
        None => String::new(),
        Some(char) => char.to_uppercase().collect::<String>() + char_1.as_str(),
    }
}

pub fn collect_cli_arg() -> String {
    // We assign arg to a match case, if no arg is provided, we exit with non 0 status code
    match std::env::args().nth(1) {
        Some(arg) => arg.to_lowercase(),
        None => {
            println!(
                "\n> Please provide a token name (see https://www.coingecko.com/ for full list)."
            );
            collect_input_arg()
        }
    }
}

pub fn collect_input_arg() -> String {
    let mut input = String::new();

    // Allowing user to input token name if not provided at runtime
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    input.trim().to_lowercase()
}
