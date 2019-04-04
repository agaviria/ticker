use std::fmt;


#[derive(Debug)]
pub struct TickerError(String);

impl TickerError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl std::error::Error for TickerError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TickerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<toml::de::Error> for TickerError {
    fn from(error: toml::de::Error) -> Self {
        TickerError(error.to_string())
    }
}

impl From<std::io::Error> for TickerError {
    fn from(error: std::io::Error) -> Self {
        TickerError(error.to_string())
    }
}

impl From<std::string::String> for TickerError {
    fn from(error: std::string::String) -> Self {
        TickerError(error.to_string())
    }
}
