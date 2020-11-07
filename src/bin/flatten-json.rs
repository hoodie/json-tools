use serde_json::Value;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn needs_escape(input: &str) -> bool {
    input.contains(|c| matches!(c, '.' | '@' | '=' | ' ' | '\x09'..='\x0d'| '0'..='9'))
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
        Value::String(s) => {
            if needs_escape(s) {
                paths.push(
                    [
                        trail,
                        dot(trail, " = "),
                        &serde_json::to_string(&s).unwrap(),
                    ]
                    .concat(),
                )
            } else {
                paths.push([trail, dot(trail, " = "), &s].concat())
            }
        }
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
                    &(if needs_escape(k) {
                        [trail, dot(trail, "."), &serde_json::to_string(&k).unwrap()].concat()
                    } else {
                        [trail, dot(trail, "."), &k].concat()
                    }),
                    paths,
                    v,
                );
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content: Option<Value> = env::args()
        .nth(1)
        .and_then(|p| File::open(p).ok())
        .map(BufReader::new)
        .map(serde_json::from_reader)
        .and_then(Result::ok);

    if let Some(content) = content {
        // flatten(&content, None, &mut list);
        let mut list = Vec::new();
        flatten("", &mut list, &content);
        println!("{}", list.join("\n"));
    }

    Ok(())
}
