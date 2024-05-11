use crate::utils;
use std::collections::HashMap;
use std::io;

pub fn get_price(token: &str, token_list: &[String; 5]) {
    if !token_list.contains(&token.to_lowercase().to_owned()) {
        println!(
            "\n> \"{}\" is not in the tokens list, do you want to make the API call anyway ? y/n",
            token
        );

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        if !["y".to_string(), "yes".to_string()].contains(&choice.trim().to_lowercase()) {
            println!("Aborting ...");
            return;
        }
    }
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        token
    );
    let resp = reqwest::blocking::get(&url)
        .expect("Failed to fetch data from the API")
        .text()
        .expect("Failed to read response body");

    let price = serde_json::from_str::<HashMap<String, HashMap<String, f64>>>(&resp)
        .expect("Failed to parse JSON");

    match price.get(token) {
        Some(data) => {
            let price = data.get("usd").unwrap().to_owned();

            println!("\n>> {}: {:.3} $\n", utils::capitalize(&token), price)
        }
        None => {
            println!("\n> Invalid token name \"{}\", see https://www.coingecko.com/ for full list.\nEnter it again: ", token);
            let new_token = utils::collect_input_arg();
            get_price(&new_token[0], &token_list)
        }
    }
}