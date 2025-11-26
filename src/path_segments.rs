use std::num::ParseIntError;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while1},
    character::complete::{char, one_of},
    combinator::{complete, map, map_res, opt, value},
    error::{context, ContextError, FromExternalError, ParseError},
    multi::many0,
    sequence::{delimited, terminated},
    Finish, IResult, Parser as _,
};

#[cfg(test)]
use crate::assert_parser;

#[derive(Debug, PartialEq)]
pub enum PathSegment<'a> {
    Name(&'a str),
    Index(usize),
}

#[test]
#[rustfmt::skip]
fn test_index_segment() {
    use PathSegment::*;
    assert_parser!(index_segment, r#"[1]"#, Index(1));
}

#[test]
#[rustfmt::skip]
fn test_empty_index_segment() {
    assert!(index_segment::<nom::error::Error<&str>>( r#"[]"#).is_err());
}

pub fn index_segment<'a, E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>>(
    input: &'a str,
) -> IResult<&'a str, PathSegment<'a>, E> {
    map(
        map_res(
            delimited(char('['), take_while1(char::is_numeric), char(']')),
            |index: &str| index.parse::<usize>(),
        ),
        PathSegment::Index,
    )
    .parse(input)
}

// ------------------------------------------------------

#[test]
#[rustfmt::skip]
fn test_name_segment() {
    use PathSegment::*;
    assert_parser!(name_segment, path_segment,  r#"abc"#, Name("abc"));
}

pub fn name_segment<'a, E>(input: &'a str) -> IResult<&'a str, PathSegment<'a>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    map(
        // default case
        take_while1(|c| c != '[' && c != '.'),
        PathSegment::Name,
    )
    .parse(input)
}

// ------------------------------------------------------

#[test]
#[rustfmt::skip]
// #[ignore = "todo"]
fn test_escaped_quoted_name_segment() {
    use PathSegment::*;
    assert_parser!(quoted_name_segment, r#"["hello\"world1"]"#, Name(r#"hello\"world1"#));
    assert_parser!(quoted_name_segment, r#"["hello \"world\" 2"]"#, Name(r#"hello \"world\" 2"#));
}

#[test]
#[rustfmt::skip]
fn test_empty_quoted_name_segment() {
    use PathSegment::*;
    assert_parser!(quoted_name_segment, path_segment, r#"[""]"#, Name(""));
}

#[test]
#[rustfmt::skip]
fn test_quoted_name_segment() {
    use PathSegment::*;
    assert_parser!(quoted_name_segment, path_segment, r#"["1"]"#, Name("1"));
    assert_parser!(quoted_name_segment, path_segment, r#"["one"]"#, Name("one"));
    assert_parser!(quoted_name_segment, path_segment, r#"["hello world"]"#, Name("hello world"));
}

pub fn quoted_name_segment<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PathSegment<'a>, E> {
    complete(map(
        alt((
            value("", tag(r#"[""]"#)),
            delimited(
                tag("[\""),
                context(
                    "escape",
                    escaped(
                        take_while1(|c| c != '\\' && c != '"'),
                        '\\',
                        one_of(r#""\n"#),
                    ),
                ),
                tag("\"]"),
            ),
        )),
        PathSegment::Name,
    ))
    .parse(input)
}

// ------------------------------------------------------

#[test]
#[rustfmt::skip]
//#[ignore = "todo"]
fn test_escaped_path_segments() {
    use PathSegment::*;
    assert_parser!(path_segments, r#"1.b.["hello \"world\""]"#, vec![Name("1"), Name("b"), Name(r#"hello \"world\""#)]);
}

#[test]
#[rustfmt::skip]
fn test_path_segments() {
    use PathSegment::*;
    assert_parser!(path_segments, r#"abc"#, vec![Name("abc")]);
    assert_parser!(path_segments, r#"abc.abc"#, vec![Name("abc"),Name("abc")]);
    assert_parser!(path_segments, r#"abc["1"]"#, vec![Name("abc"),Name("1")]);
    assert_parser!(path_segments, r#"["1"]["2"]"#, vec![Name("1"), Name("2")]);
    assert_parser!(path_segments, r#"["1"]["2"][3]"#, vec![Name("1"), Name("2"), Index(3)]);
    assert_parser!(path_segments, r#"["1"]["2"][3]"#, vec![Name("1"), Name("2"), Index(3)]);
    assert_parser!(path_segments,
        r#"arrays[2]["special-things"].special"#, vec![
            Name("arrays"), Index(2), Name("special-things"), Name("special")
        ]);
    assert_parser!(path_segments, r#"1.b.c.d"#, vec![Name("1"), Name("b"), Name("c"), Name("d")]);
    assert_parser!(path_segments, r#"1.b.c["d"]"#, vec![Name("1"), Name("b"), Name("c"), Name("d")]);
    assert_parser!(path_segments, r#"1.b.["hello world{}[]<>"].d"#, vec![Name("1"), Name("b"), Name("hello world{}[]<>"), Name("d")]);
    assert_parser!(path_segments, r#"1.b.["hello world"].d[9]"#, vec![Name("1"), Name("b"),      Name("hello world"), Name("d"), Index(9)]);
    assert_parser!(path_segments, r#"1.b.["hello world"].d[9].23"#, vec![Name("1"), Name("b"), Name("hello world"), Name("d"), Index(9), Name("23")]);

}

pub fn path_segment<
    'a,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
>(
    input: &'a str,
) -> IResult<&'a str, PathSegment<'a>, E> {
    alt((index_segment, quoted_name_segment, name_segment)).parse(input)
}

pub fn path_segments<
    'a,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
>(
    input: &'a str,
) -> IResult<&'a str, Vec<PathSegment<'a>>, E> {
    many0(terminated(path_segment, opt(tag(".")))).parse(input)
}

pub fn parse(input: &str) -> Vec<PathSegment<'_>> {
    path_segments::<nom::error::Error<&str>>(input)
        .finish()
        .unwrap()
        .1
}
