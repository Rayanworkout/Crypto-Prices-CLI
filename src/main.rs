mod get_price;
mod utils;

#[tokio::main]
async fn main() {
    let known_tokens: Vec<String> = vec![
        "ethereum",
        "bitcoin",
        "solana",
        "dogecoin",
        "avalanche-2",
        "polkadot",
        "chainlink",
    ]
    .iter()
    .map(|t| t.to_string())
    .collect();

    let tokens = utils::collect_cli_args();

    for token in tokens {
        if !known_tokens.contains(&token.to_lowercase()) {
            match utils::find_closest_match(&token, &known_tokens) {
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
                                false => println!("> Aborting API call for \"{}\"", &token),
                            }
                        }
                    }
                }
                None => println!(
                    "> No matching cryptocurrency found for \"{}\", aborting ...",
                    &token
                ),
            }
        } else {
            get_price::get_price(&token).await
        }
    }
}
