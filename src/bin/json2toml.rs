fn main() -> Result<(), Box<dyn std::error::Error>> {
    json_tools::with_json_value(|content| {
        let reserialized = toml::Value::try_from(&content)?;

        println!("{}", reserialized);

        Ok(())
    })
}
