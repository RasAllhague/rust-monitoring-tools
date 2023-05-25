use std::{env, num::ParseIntError};

pub struct AppConfigurations {
    pub bot_token: String,
    pub server_url: String,
    pub api_token: String,
    pub read_token: String,
    pub allowed_users: Vec<u64>,
}

#[derive(Clone)]
pub struct BotConfig {
    server_url: String,
    api_token: String,
    read_token: String,
    allowed_users: Vec<u64>,
}

impl BotConfig {
    pub fn new(api_token: &str, read_token: &str, server_url: &str, allowed_users: &[u64]) -> Self {
        Self {
            api_token: String::from(api_token),
            read_token: String::from(read_token),
            server_url: String::from(server_url),
            allowed_users: allowed_users.to_vec(),
        }
    }

    pub fn api_token(&self) -> &str {
        self.api_token.as_ref()
    }

    pub fn read_token(&self) -> &str {
        self.read_token.as_ref()
    }

    pub fn server_url(&self) -> &str {
        self.server_url.as_ref()
    }

    pub fn allowed_users(&self) -> &[u64] {
        self.allowed_users.as_ref()
    }
}

impl AppConfigurations {
    pub fn from_env() -> AppConfigurations {
        let token = env::var("MONITORING_BOT_TOKEN").expect("Expected a token in the environment!");
        let server_url =
            env::var("MONITORING_SERVER_URL").expect("Expected server url in environment!");
        let api_token = env::var("MONITORING_API_KEY").expect("Expected api token in environment!");
        let read_token =
            env::var("MONITORING_READ_KEY").expect("Expected read key in environment!");
        let allowed_users =
            env::var("MONITORING_ALLOWED_USERS").expect("Expected allowed users in environment!");
        let allowed_users = allowed_users
            .split(';')
            .map(|x| x.parse::<u64>())
            .collect::<Result<Vec<u64>, ParseIntError>>()
            .expect("Invalid users in allowed users!");

        AppConfigurations {
            bot_token: token,
            server_url,
            api_token,
            read_token,
            allowed_users,
        }
    }
}
