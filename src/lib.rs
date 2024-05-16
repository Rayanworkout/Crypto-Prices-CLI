pub const CRYPTO_LIST: &str = include_str!("utils/cryptocurrencies.txt");

/// Capitalize a given string reference
pub trait Capitalize {
    fn capitalize(&self) -> String;
}

impl Capitalize for &str {
    fn capitalize(&self) -> String {
        let mut char_1 = self.chars();
    
        match char_1.next() {
            None => String::new(),
            Some(char) => char.to_uppercase().collect::<String>() + char_1.as_str(),
        }
    }
}

pub use self::utils::*;

mod utils;
