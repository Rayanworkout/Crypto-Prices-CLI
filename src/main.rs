mod get_price;
mod utils;

fn main() {
    let known_tokens =
        ["ethereum", "bitcoin", "solana", "dogecoin", "avalanche-2"].map(|t| String::from(t));

    let tokens = utils::collect_cli_args();

    for token in tokens {
        get_price::get_price(&token, &known_tokens);
    }
}
