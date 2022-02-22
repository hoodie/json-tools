use serde_json::Value;

use std::error::Error;

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
        Value::Bool(s) => {
            paths.push([trail, dot(trail, " = "), if *s { "true" } else { "false" }].concat())
        }
        Value::Number(s) => paths.push(format!("{}{}{}", trail, dot(trail, " = "), s)),
        Value::Null => paths.push([trail, dot(trail, " = "), "null"].concat()),
        Value::Array(a) => {
            for (i, v) in a.iter().enumerate() {
                flatten(&[trail, "[", &i.to_string(), "]"].concat(), paths, v);
            }
        }
        Value::Object(o) => {
            for (k, v) in o {
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
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    json_tools::with_json_value(|content| {
        let mut list = Vec::new();
        flatten("", &mut list, &content);
        println!("{}", list.join("\n"));
        Ok(())
    })
}
