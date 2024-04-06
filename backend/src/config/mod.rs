use serde::Deserialize;
use std::fs::read_to_string;

//TODO look into better typings for things like URLs (I see there are some provided from both actix and openidconnect)
#[derive(Deserialize)]
pub struct OIDCConfig {
    pub base_url: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub addr: String,
    pub port: u16,
    pub oidc: OIDCConfig
}

pub fn parse_config() -> Config {
    match read_to_string("config.toml") {
        Ok(config_string) => match toml::from_str::<Config>(&config_string) {
            Ok(config) => config,
            Err(error) => panic!("Error parsing config.toml: {:?}", error)
        }
        Err(error) => panic!("Could not read config.toml App will not start: {:?}", error)
    }
}