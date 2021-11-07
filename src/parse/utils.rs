use aho_corasick::AhoCorasick;
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
        take_while(|c: char| {
            c == '.' || c == ',' || c == '/' || c == '_' || c == '-' || c.is_alphanumeric()
        })(input)
    }
}
pub fn property_key_cow<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&str, &'a str, E> {
    property_key(input)
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
/// # use icalendar::parse::normalize;
/// let line = r#"this
///   gets
///   wrapped
///   in
///   a weird
///   way"#;
///
/// assert_eq!(normalize(line), "this gets wrapped in a weird way")
/// ```
pub fn normalize(input: &str) -> String {
    unfold(&simplify_line_endings(input))
}

fn unfold(input: &str) -> String {
    let mut output = Vec::<u8>::new();

    // unfold
    AhoCorasick::new(&["\n "])
        .stream_replace_all(input.as_bytes(), &mut output, &[""])
        .unwrap();

    String::from_utf8(output).unwrap()
}

fn simplify_line_endings(input: &str) -> String {
    let mut output = Vec::<u8>::new();

    // unfold
    AhoCorasick::new(&["\r\n"])
        .stream_replace_all(input.as_bytes(), &mut output, &["\n"])
        .unwrap();

    String::from_utf8(output).unwrap()
}

#[test]
fn test_unfold() {
    let input = "1 hello world\r\n2 hello\r\n  world\r\n3 hello world\r\n4 hello world";

    let expected = r#"1 hello world
2 hello world
3 hello world
4 hello world"#;
    assert_eq!(normalize(input), expected);
}

/// this is actually also allowed by the spec
#[test]
#[ignore]
fn test_unfold2() {
    let input = "1 hello world\r\n2 hello\r\n  world\r\n3 hello world\r\n4 hello world";

    let expected = r#"1 hello world
2 hello world
 3 hello world
  4 hello world"#;
    assert_eq!(normalize(input), expected);
}
