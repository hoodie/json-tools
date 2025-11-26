use std::fmt;

pub fn print_result<T: fmt::Debug>(input: &str, rest: &str, result: &T) {
    println!(
        "INPUT: {:?}\nLEFT:  {:?}\nRESULT: {:#?}",
        input, rest, result
    );
}

#[macro_export]
macro_rules! assert_parser {
    // ($call:expr, $expected:expr) => {{
    //     let result = $call.unwrap().1;
    //     assert_eq!(result, $expected);
    // }};
    ($parser1:ident, $parser2:ident, $line:expr, $expectation:expr) => {{
        assert_parser!($parser1, $line, $expectation);
        assert_parser!($parser2, $line, $expectation);
    }};

    ($parser:ident, $line:expr, $expectation:expr) => {{
        match $parser::<(_, nom::error::ErrorKind)>(&$line) {
            Ok((rest, parsed)) => {
                $crate::assert::print_result($line, &rest, &parsed);
                pretty_assertions::assert_eq!(
                    parsed,
                    $expectation,
                    "{:?} not parsed as expected",
                    $line
                );
                assert!(rest.is_empty(), "not parsed completely");
            }
            Err(error) => {
                assert!(false, "{}", error);
            }
        }
    }};

    ($parser:ident, $line:expr, $expectation:expr, print) => {{
        let (rest, parsed) = $parser::<(_, nom::error::ErrorKind)>(&$line).unwrap();
        $crate::assert::print_result($line, &rest, &parsed);
        pretty_assertions::assert_eq!(parsed, $expectation, "{:?} not parsed as expected", $line);
        assert!(rest.is_empty(), "not parsed completely");

        let serialized = parsed.to_string();
        pretty_assertions::assert_eq!($line, serialized);
    }};
}
