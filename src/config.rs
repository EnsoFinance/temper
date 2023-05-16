use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub fork_url: Option<String>,
    pub etherscan_key: Option<String>,
    pub api_key: Option<String>,
}

pub fn config() -> Config {
    dotenv().ok();

    load_config()
}

fn load_config() -> Config {
    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16.");
    let fork_url = std::env::var("FORK_URL").ok().filter(|k| !k.is_empty());
    let etherscan_key = std::env::var("ETHERSCAN_KEY")
        .ok()
        .filter(|k| !k.is_empty());
    let api_key = std::env::var("API_KEY").ok().filter(|k| !k.is_empty());

    Config {
        fork_url,
        port,
        etherscan_key,
        api_key,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "PORT must be a valid u16.")]
    fn test_config_port_number() {
        temp_env::with_var("PORT", Some("not a number"), || {
            super::load_config();
        });
    }

    #[test]
    fn test_config_fork_url() {
        temp_env::with_vars([("FORK_URL", Some("a"))], || {
            let config = super::load_config();
            assert_eq!(config.fork_url, Some("a".to_string()));
        });

        temp_env::with_vars([("FORK_URL", Some(""))], || {
            let config = super::load_config();
            assert_eq!(config.fork_url, None);
        });

        temp_env::with_vars_unset([("FORK_URL")], || {
            let config = super::load_config();
            assert_eq!(config.fork_url, None);
        });
    }

    #[test]
    fn test_config_etherscan_key() {
        temp_env::with_vars([("ETHERSCAN_KEY", Some("a"))], || {
            let config = super::load_config();
            assert_eq!(config.etherscan_key, Some("a".to_string()));
        });

        temp_env::with_vars([("ETHERSCAN_KEY", Some(""))], || {
            let config = super::load_config();
            assert_eq!(config.etherscan_key, None);
        });

        temp_env::with_vars_unset([("ETHERSCAN_KEY")], || {
            let config = super::load_config();
            assert_eq!(config.etherscan_key, None);
        });
    }

    #[test]
    fn test_config_api_key() {
        temp_env::with_vars([("API_KEY", Some("a"))], || {
            let config = super::load_config();
            assert_eq!(config.api_key, Some("a".to_string()));
        });

        temp_env::with_vars([("API_KEY", Some(""))], || {
            let config = super::load_config();
            assert_eq!(config.api_key, None);
        });

        temp_env::with_vars_unset([("API_KEY")], || {
            let config = super::load_config();
            assert_eq!(config.api_key, None);
        });
    }
}
