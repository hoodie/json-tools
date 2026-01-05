use json_tools::path_segments::PathSegment;
use serde_json::{Map, Value};

trait ValueExt {
    fn insert_into(&mut self, ps: &PathSegment, value: Value);
    fn get_ps(&self, ps: &PathSegment) -> Option<&Value>;
    fn get_ps_mut(&mut self, ps: &PathSegment) -> Option<&mut Value>;
}

impl ValueExt for Value {
    fn insert_into(&mut self, ps: &PathSegment, value: Value) {
        match (self, ps) {
            (Value::Object(map), PathSegment::Name(name)) => {
                map.insert(name.to_string(), value);
            }
            (Value::Array(vec), PathSegment::Index(_)) => {
                vec.push(value);
            }
            _ => unreachable!(),
        }
    }

    fn get_ps(&self, ps: &PathSegment) -> Option<&Value> {
        match ps {
            PathSegment::Name(name) => self.get(*name),
            PathSegment::Index(index) => self.get(index),
        }
    }

    fn get_ps_mut(&mut self, ps: &PathSegment) -> Option<&mut Value> {
        match ps {
            PathSegment::Name(s) => self.get_mut(s),
            PathSegment::Index(i) => self.get_mut(i),
        }
    }
}

fn pair_or_single<'a>(line: &'a str, delimiter: &'static str) -> (&'a str, Option<&'a str>) {
    if let Some((key, val)) = line.split_once(delimiter) {
        (key.trim(), Some(val.trim()))
    } else {
        (line.trim(), None)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    json_tools::with_string_content(|content| {
        let unflattened: Vec<(Vec<PathSegment>, Option<&str>)> = content
            .lines()
            .map(|l| pair_or_single(l, "="))
            .map(|(k, v)| (json_tools::path_segments::parse(k), v))
            //.map(|(k, v)| (parse_path(k).collect::<Vec<_>>(), v))
            .collect::<Vec<_>>();

        let mut root = if unflattened
            .first()
            .and_then(|(segments, _)| segments.first())
            .map(|seg| matches!(seg, PathSegment::Index(_)))
            .unwrap_or(false)
        {
            Value::Array(Vec::new())
        } else {
            Value::Object(Map::new())
        };

        for (segments, value) in unflattened {
            let value = value.expect("no single case!!!");
            let mut cursor = &mut root;

            for (current_path, next_path) in segments
                .iter()
                .zip(segments.iter().map(Some).chain(Some(None)).skip(1))
            {
                match (cursor.get_ps(current_path), next_path) {
                    (None, None) => {
                        cursor.insert_into(current_path, serde_json::from_str(value).unwrap());
                    }
                    (None, Some(&PathSegment::Name(_))) => {
                        cursor.insert_into(current_path, Value::Object(Map::new()));
                        cursor = cursor.get_ps_mut(current_path).unwrap();
                    }
                    (None, Some(&PathSegment::Index(_))) => {
                        cursor.insert_into(current_path, Value::Array(Vec::new()));
                        cursor = cursor.get_ps_mut(current_path).unwrap();
                    }
                    (Some(next), Some(_)) => {
                        assert!(next.is_object() || next.is_array());
                        cursor = cursor.get_ps_mut(current_path).unwrap();
                    }
                    (Some(_), None) => todo!(),
                }
            }
        }

        let json = serde_json::to_string_pretty(&root).unwrap();
        println!("{}", json);

        Ok(())
    })
}
