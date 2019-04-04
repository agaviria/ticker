#![feature(result_map_or_else)]
#![type_length_limit = "33554432"]
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod logger;
pub mod conf;


fn main() {
    let _ = conf::load(Some("prod"));
}
