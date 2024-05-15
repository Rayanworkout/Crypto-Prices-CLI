use std::collections::HashMap;
use std::io;
use strsim::levenshtein;

/// Function to capitalize a given string reference.
///
/// If string reference is null, returns a new string.
///
/// Returns a new string with the first letter capitalized.
pub fn capitalize(token: &str) -> String {
    let mut char_1 = token.chars();

    match char_1.next() {
        None => String::new(),
        Some(char) => char.to_uppercase().collect::<String>() + char_1.as_str(),
    }
}

/// Function to collect command line arguments.
///
/// It collects the arguments given at runtime and returns a vector of lowercase strings.
pub fn collect_cli_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!(
            "\n> Please provide at least one token name (see https://www.coingecko.com/ for full list)."
        );
        collect_input_arg()
    } else {
        args[1..]
            .iter()
            .map(|str| str.to_lowercase())
            .collect::<Vec<String>>()
    }
}

/// Function to collect input from the user.
///
/// Returns a vector with the input string to lowercase.
pub fn collect_input_arg() -> Vec<String> {
    let mut input = String::new();

    // Allowing user to input token name if not provided at runtime
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    vec![input.trim().to_lowercase()]
}

/// Function to confirm a choice from the user.
///
/// The user is prompted to confirm a choice with a yes or no.
///
/// Returns yes if the user confirms with a yes, otherwise false.
/// ```
/// match confirm_choice() {
///    true => println!("\n> User chose yes ..."),
///    false => println!("\n> User chose no ..."),
/// ```
///
pub fn confirm_choice() -> bool {
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let yes = ["y", "yes"].contains(&choice.trim().to_lowercase().as_str());

    if yes {
        true
    } else {
        false
    }
}

/// Function to find the closest match of a given string in a list of strings.
///
/// Here we find the closest token name to the input string.
/// ```
/// match find_closest_match(&string, &string_list) {
///     Some(closest_name) => {
///         println!(
///             "> \"{}\" was not found. Did you mean {}? (y/n)",
///             &string, &closest_name
///         );
/// ```
pub fn find_closest_match<'a>(input: &'a str, target_list: &'a Vec<&str>) -> Option<&'a str> {
    let mut similarities: HashMap<&str, usize> = HashMap::new();

    for name in target_list {
        let distance = levenshtein(input, &name);
        similarities.insert(&name, distance);
    }

    let (closest_name, _) = similarities.iter().min_by_key(|&(_, &distance)| distance)?;

    Some(closest_name)
}

/// Function to map the user input to a list of known values.
///
/// It is used to gain some time and allow user to use aliases.
/// For example, "eth" is an alias for "ethereum".
/// This way, we don't have to use the find_closest_match function.
///
/// Returns either the found token or None.
pub fn map_input_to_value(input: &str) -> Option<String> {
    let mut known_variants: HashMap<String, Vec<String>> = HashMap::new();

    known_variants.insert(
        "ethereum".to_string(),
        vec!["eth", "ether", "e"]
            .iter()
            .map(|e| e.to_string())
            .collect(),
    );

    known_variants.insert("bitcoin".to_string(), vec!["btc".to_string()]);

    for (key, value) in &known_variants {
        if value.contains(&input.to_owned()) {
            return Some(key.to_owned());
        }
    }

    None
}
