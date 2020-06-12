use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::env;

fn main() -> Result<(), Box<dyn Error>>{

    let content: Option<serde_json::Value> = env::args()
        .nth(1)
        .and_then(|p| File::open(p).ok())
        .map(BufReader::new)
        .map(serde_json::from_reader)
        .and_then(Result::ok);

    let reserialized = serde_json::to_string(&content)?;


    println!("export const data = '{}';", reserialized);

    Ok(())
}
