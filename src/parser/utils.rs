use nom::{
    bytes::complete::{tag_no_case, take_while},
    character::complete::line_ending,
    combinator::complete,
    error::{ContextError, ParseError},
    multi::many0,
    sequence::{delimited, preceded},
    Err, IResult, Parser,
};
#[cfg(test)]
use pretty_assertions::assert_eq;

// TODO: how do I express <<alpha_or_dash, but not "END">>
pub fn property_key<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &str, E> {
    if &input[0..=2] == "END" || &input[0..=4] == "BEGIN" {
        IResult::Err(Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Satisfy,
        )))
    } else {
        valid_key_sequence(input)
    }
}

pub fn valid_key_sequence<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &str, E> {
    take_while(|c: char| {
        c == '.' || c == ',' || c == '/' || c == '_' || c == '-' || c.is_alphanumeric()
    })(input)
}

pub fn line<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    prefix: &'a str,
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    line_separated(complete(preceded(tag_no_case(prefix), f)))
}

pub fn line_separated<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    delimited(many0(line_ending), f, many0(line_ending))
}

/// Normalize content lines.
///
/// This simplifies line endings and unfolds breaks to simplify parsing.
/// iCal specifies that content may be folded and to fit into a certain
/// length, which must be undone before parsing.
///
/// This is a copying operation.
///
/// # Example
///
/// ```
/// # use icalendar::parser::unfold;
/// #[rustfmt::skip]
/// let line = r#"this g
///   ets w
///   rapped i
///   n a w
///    eird w
///   ay"#;
///
/// assert_eq!(unfold(line), "this gets wrapped in a weird way")
/// ```
pub fn unfold(input: &str) -> String {
    input
        .split("\r\n ")
        .flat_map(|l| l.split("\n "))
        .map(str::trim_start)
        .collect()
}

#[test]
fn test_unfold1() {
    let input = "1 hello world\r\n2 hello \r\n   world\r\n3 hello \r\n world\r\n4 hello world";
    let expected = "1 hello world\r\n2 hello world\r\n3 hello world\r\n4 hello world";
    assert_eq!(unfold(input), expected);
}

/// this is actually also allowed by the spec
#[test]
fn test_unfold2() {
    let input1 = "1 hello world\n2 hello \n  world\n3 hello world\n4 hello world";
    let input2 = "1 hello world\r\n2 hello \r\n   world\r\n3 hello \r\n world\r\n4 hello world";

    let expected = vec![
        "1 hello world",
        "2 hello world",
        "3 hello world",
        "4 hello world",
    ];

    assert_eq!(unfold(input1).lines().collect::<Vec<_>>(), expected);
    assert_eq!(unfold(input2).lines().collect::<Vec<_>>(), expected);
}
