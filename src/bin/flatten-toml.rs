use std::error::Error;

use toml::{Table, Value};

fn needs_quotes(input: &str) -> bool {
    input.contains(|c| matches!(c, '.' | '-' | '@' | '=' | ' ' | '\x09'..='\x0d'| '0'..='9'))
}

fn dot<'a>(input: &str, sep: &'a str) -> &'a str {
    if input.is_empty() {
        ""
    } else {
        sep
    }
}

fn flatten_table(trail: &str, paths: &mut Vec<String>, table: &Table) {
    for (k, v) in table {
        flatten(
            &(if needs_quotes(k) {
                [trail, "[", &serde_json::to_string(&k).unwrap(), "]"].concat()
            } else {
                [trail, dot(trail, "."), k].concat()
            }),
            paths,
            v,
        );
    }
}
fn flatten(trail: &str, paths: &mut Vec<String>, value: &Value) {
    match value {
        Value::String(s) => paths.push(
            [
                trail,
                dot(trail, " = "),
                &serde_json::to_string(&s).unwrap(),
            ]
            .concat(),
        ),
        Value::Boolean(s) => {
            paths.push([trail, dot(trail, " = "), if *s { "true" } else { "false" }].concat())
        }
        Value::Integer(s) => paths.push(format!("{}{}{}", trail, dot(trail, " = "), s)),
        Value::Float(s) => paths.push(format!("{}{}{}", trail, dot(trail, " = "), s)),
        Value::Array(a) => {
            for (i, v) in a.iter().enumerate() {
                flatten(&[trail, "[", &i.to_string(), "]"].concat(), paths, v);
            }
        }
        Value::Table(table) => flatten_table(trail, paths, table),
        Value::Datetime(datetime) => {
            paths.push(format!("{}{}{}", trail, dot(trail, " = "), datetime))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    json_tools::with_toml_value(|content| {
        let mut list = Vec::new();
        flatten_table("", &mut list, &content);
        println!("{}", list.join("\n"));
        Ok(())
    })
}
