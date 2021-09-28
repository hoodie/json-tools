fn main() -> Result<(), Box<dyn std::error::Error>> {
    json_tools::with_json_value(|content| {
        let reserialized = serde_json::to_string(&content)?;

        println!("export const data = '{}';", reserialized);

        Ok(())
    })
}
