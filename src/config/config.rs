use std::env;
use dotenv::dotenv;

pub struct Config {
    pub discord_bot_token: String,
    pub is_json_pretty: bool
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        let discord_bot_token = env::var("DISCORD_BOT_TOKEN").expect("'DISCORD_BOT_TOKEN'를 .env에 추가해주세요");
        let is_json_pretty: bool = env::var("IS_JSON_PRETTY").unwrap_or("false".to_string()).parse().unwrap();
        Config { discord_bot_token, is_json_pretty }
    }
}