use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub alchemy_key: String,
    pub port: u16,
    pub etherscan_key: Option<String>,
    pub api_key: Option<String>,
}

pub fn get_config() -> Config {
    dotenv().ok();

    let alchemy_key = std::env::var("ALCHEMY_KEY")
        .ok()
        .filter(|k| !k.is_empty())
        .expect("ALCHEMY_KEY must be set.");
    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number.");
    let etherscan_key = std::env::var("ETHERSCAN_KEY")
        .ok()
        .filter(|k| !k.is_empty());
    let api_key = std::env::var("API_KEY").ok().filter(|k| !k.is_empty());

    Config {
        alchemy_key,
        port,
        etherscan_key,
        api_key,
    }
}
