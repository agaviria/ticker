use std::env;
use std::path::{
    PathBuf,
};
use std::fs;
use std::io::{
    Read,
};

use toml;
use crate::error::TickerError;

const CFG_FILE: &'static str = "ticker.toml";

// #[derive(Debug, Deserialize)]
// pub struct TDAmeritradeConfig {
// }

#[derive(Debug, Deserialize)]
pub struct Config {
    pub auth_url : String,
    pub token_url : String,
    // pub ameritrade : TDAmeritradeConfig,
}

impl Config {
    // maps configuration directory to ~/.config/ticker/
    fn config_dir() -> PathBuf {
        env::var("XDG_CONFIG_HOME")
            .map_or_else(
                |_| env::var("HOME").expect("") + "/.config/" + env!("CARGO_PKG_NAME"),
                |path| path + "/" + env!("CARGO_PKG_NAME"),
                )
            .into()
    }

    // open configuration file and read to string buffer.
    //
    // usage: `Ameritrade::read_cfg_file().unwrap()`
    pub fn read_cfg_file() -> Result<Config, TickerError> {
        // sets configuration file to ~/.config/ticket/ticker.toml
        let cfg_file = fs::File::open(Self::config_dir().join(CFG_FILE))?;

        let contents = cfg_file.read_to_string(&mut buf).?;
        let result = toml::from_str(&buf);
        Ok(result)
    }

}
// pub fn load_config() -> Result<Ameritrade, failure::Error> {
//     let mut cfg = Self::read_cfg_file();

//     // Override global.auth_url via ENV.
//     if let Ok(auth_url) = std::env::var("AUTH_URL") {
//         cfg.auth_url = auth_url;
//     }

//     // Override global.chain via command-line.
//     if let Ok(token_url) = std::env::var("TOKEN_URL") {
//         cfg.token_url = token_url;
//     }
//     Ok(cfg)
// }
