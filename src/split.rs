use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while},
    character::complete::one_of,
    combinator::complete,
    error::{ContextError, ParseError},
    multi::separated_list0,
    sequence::delimited,
    Finish, IResult,
};

#[test]
fn test_split_smart() {
    assert_eq!(split_smart(r#""1"."b"."c"."d""#), vec!["1", "b", "c", "d"]);
    assert_eq!(
        split_smart(r#"1.b."hello world{}[]<>".d"#),
        vec!["1", "b", "hello world{}[]<>", "d"]
    );
    assert_eq!(
        split_smart(r#"1.b."hello world".d[9]"#),
        vec!["1", "b", "hello world", "d[9]"]
    );
    assert_eq!(
        split_smart(r#"1.b."hello world".d[9]."23""#),
        vec!["1", "b", "hello world", "d[9]", "23"]
    );
}

pub fn parse_split_smart<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<&'a str>, E> {
    complete(separated_list0(
        tag("."),
        alt((
            delimited(
                tag("\""),
                escaped(take_while(|c| c != '"'), '\\', one_of("\"n\\")),
                tag("\""),
            ),
            take_while(|c| c != '.'),
        )),
    ))(input)
}

pub fn split_smart(input: &str) -> Vec<&str> {
    parse_split_smart::<nom::error::Error<&str>>(input)
        .finish()
        .unwrap()
        .1
    // {
    //     Ok(parsed) => parsed.1,
    //     Err(error) => {
    //         println!("{}", error);
    //         Vec::new()
    //     }
    // }
}
