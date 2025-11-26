use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::{env, io};

pub mod path_segments;

#[cfg(all(test))]
#[macro_use]
mod assert;

fn json_value_from_arg(arg: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let (mut std_in, mut file);
    let readable: &mut dyn io::Read = if arg == "-" {
        std_in = io::stdin();
        &mut std_in
    } else {
        file = File::open(arg)?;
        &mut file
    };

    Ok(serde_json::from_reader(readable)?)
}

fn yaml_value_from_arg(arg: &str) -> Result<serde_yaml::Value, Box<dyn Error>> {
    let (mut std_in, mut file);
    let readable: &mut dyn io::Read = if arg == "-" {
        std_in = io::stdin();
        &mut std_in
    } else {
        file = File::open(arg)?;
        &mut file
    };

    Ok(serde_yaml::from_reader(readable)?)
}

fn toml_value_from_arg(arg: &str) -> Result<toml::Table, Box<dyn Error>> {
    Ok(toml::Table::from_str(&string_from_arg(arg)?)?)
}

fn string_from_arg(arg: &str) -> Result<String, Box<dyn Error>> {
    if arg == "-" {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        Ok(content)
    } else {
        Ok(std::fs::read_to_string(arg)?)
    }
}

pub fn with_string_content<F>(closure: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(&str) -> Result<(), Box<dyn Error>>,
{
    if let Some(arg) = env::args().nth(1) {
        closure(&self::string_from_arg(&arg)?)?;
    } else {
        println!("No argument given. Either pass file name of \"-\" for stdin.")
    }

    Ok(())
}
pub fn with_json_value<F>(closure: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(serde_json::Value) -> Result<(), Box<dyn Error>>,
{
    if let Some(arg) = env::args().nth(1) {
        let content = self::json_value_from_arg(&arg)?;

        closure(content)?;
    } else {
        println!("No argument given. Either pass file name of \"-\" for stdin.")
    }

    Ok(())
}

pub fn with_yaml_value<F>(closure: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(serde_yaml::Value) -> Result<(), Box<dyn Error>>,
{
    if let Some(arg) = env::args().nth(1) {
        let content = self::yaml_value_from_arg(&arg)?;

        closure(content)?;
    } else {
        println!("No argument given. Either pass file name of \"-\" for stdin.")
    }

    Ok(())
}

pub fn with_toml_value<F>(closure: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(toml::Table) -> Result<(), Box<dyn Error>>,
{
    if let Some(arg) = env::args().nth(1) {
        let content = self::toml_value_from_arg(&arg)?;

        closure(content)?;
    } else {
        println!("No argument given. Either pass file name of \"-\" for stdin.")
    }

    Ok(())
}
