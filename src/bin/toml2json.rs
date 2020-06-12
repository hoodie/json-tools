use serde_json;

use std::env;
use std::error::Error;
use std::fs::{read_to_string};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let content: Option<toml::Value> = env::args()
        .nth(1)
        .map(|p| read_to_string(&p).unwrap())
        .map(|s| toml::Value::from_str(&s))
        .and_then(Result::ok);

    let reserialized = serde_json::to_string_pretty(&content)?;

    println!("{}", reserialized);

    Ok(())
}
