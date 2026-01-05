fn main() -> Result<(), Box<dyn std::error::Error>> {
    json_tools::with_json_value(|content| {
        let reserialized = json5::to_string(&content)?;

        println!("{}", reserialized);

        Ok(())
    })
}
