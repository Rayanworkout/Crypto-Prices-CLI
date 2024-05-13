mod get_price;
mod utils;

#[tokio::main]
async fn main() {
    const CRYPTO_LIST: &str = include_str!("cryptocurrencies.txt");

    let known_tokens: Vec<&str> = CRYPTO_LIST.trim().split('\n').collect();

    let tokens = utils::collect_cli_args();

    for token in tokens {
        if !known_tokens.contains(&token.to_lowercase().as_str()) {
            match utils::map_input_to_value(&token) {
                Some(found_value) => get_price::get_price(&found_value).await,
                None => match utils::find_closest_match(&token, &known_tokens) {
                    Some(closest_name) => {
                        println!(
                            "> \"{}\" was not found. Did you mean {}? (y/n)",
                            &token, &closest_name
                        );

                        match utils::confirm_choice() {
                            true => get_price::get_price(&closest_name).await,
                            false => {
                                println!("\n> Do you want to make the API call anyway ? y/n",);

                                match utils::confirm_choice() {
                                    true => get_price::get_price(&token).await,
                                    false => println!("> Aborting API call for \"{}\".", &token),
                                }
                            }
                        }
                    }
                    None => println!(
                        "> No matching cryptocurrency found for \"{}\", aborting ...",
                        &token
                    ),
                },
            }
        } else {
            get_price::get_price(&token).await
        }
    }
}
