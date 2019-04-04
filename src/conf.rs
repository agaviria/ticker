use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use toml;

use crate::logger;

#[derive(Deserialize)]
pub struct Config {
    pub brokerage: TDAmeritradeConfig,
}

#[derive(Deserialize, Clone)]
pub struct TDAmeritradeConfig {
    pub auth_url: String,
    pub token_url: String,
}

pub fn load(stage_env: Option<&str>) -> Config {
    // sets path to XDG_CONFIG_HOME
    let config_path = env::var("HOME").expect("") + "/.config/" +
        env!("CARGO_PKG_NAME").into() + "/" + env!("CARGO_PKG_NAME").into();

    let stage_env = format!(
        "{}_{}_config.toml",
        config_path,
        match stage_env {
            Some(env) => env,
            None => unreachable!(),
        }
    );

    let config = {
        match File::open(&Path::new(&format!("{}_config.toml", config_path))) {
            Err(_) => match File::open(&Path::new(&stage_env)) {
                Err(_) => format!("{}_dev_config.toml", config_path),
                Ok(_) => stage_env,
            },
            Ok(_) => format!("{}_config.toml", config_path),
        }
    };

    let path = Path::new(&config);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open {}: {}", display, e.description()),
        Ok(file) => file,
    };

    let mut configuration = String::new();
    match file.read_to_string(&mut configuration) {
        Err(e) => panic!("Couldn't read {}: {}", display, e.description()),
        Ok(_) => {
            logger::log(format!("{} loaded correctly.", display));

            let variables: HashMap<_, _> = env::vars().collect();
            for (key, value) in variables {
                configuration =
                    configuration.replace(&format!("\"${}\"", key), &format!("\"{}\"", value));
            }

            toml::from_str(&configuration).expect("Couldn't load the configuration file.")
        }
    }
}
