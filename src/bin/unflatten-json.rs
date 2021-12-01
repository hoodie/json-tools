use serde_json::Value;

use std::error::Error;

#[derive(Debug)]
enum PathSegment<'a> {
    Name(&'a str),
    Index(u32),
}

fn from_utf8<'a>(bytes: &'a [u8]) -> Option<&'a str> {
    std::str::from_utf8(bytes).ok().filter(|s| !s.is_empty())
}

fn parse_path<'a>(raw: &'a str) -> impl Iterator<Item = PathSegment<'a>> {
    raw.split('.')
        .filter_map(|seg| match seg.as_bytes() {
            [start @ .., b']'] => {
                //ends on index bracked
                from_utf8(start).map(|s| pair_or_single(s, "["))
            }
            [bytes @ ..] => from_utf8(bytes).map(|s| (s, None)),
        })
        .map(|(seg, i)| {
            Some(PathSegment::Name(seg))
                .into_iter()
                .chain(i.map(|i| i.parse::<u32>().unwrap()).map(PathSegment::Index))
        })
        .flatten()
}
fn parse_path_collected<'a>(raw: &'a str) -> Vec<PathSegment<'a>> {
    parse_path(raw).collect()
}

#[test]
fn test_path() {
    dbg!(parse_path_collected("a.b.c[1]"));
    dbg!(parse_path_collected(r#""string""#));
}

fn pair_or_single<'a>(line: &'a str, delimiter: &'static str) -> (&'a str, Option<&'a str>) {
    if let Some((key, val)) = line.split_once(delimiter) {
        (key.trim(), Some(val.trim()))
    } else {
        (line.trim(), None)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    json_tools::with_string_content(|content| {
        let pairs = content
            .lines()
            .map(|l| pair_or_single(l, "="))
            .collect::<Vec<_>>();

        // unflatten("", &mut list, &content);
        println!("{:#?}", pairs);

        Ok(())
    })
}
