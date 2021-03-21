use std::fmt;

use aho_corasick::AhoCorasick;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::line_ending,
    combinator::complete,
    error::{ParseError, VerboseError},
    multi::many0,
    sequence::{delimited, preceded},
    Err, IResult, Parser,
};
#[cfg(test)]
use pretty_assertions::assert_eq;

#[derive(Debug)]
struct UnexpectedBeginOrEnd;

impl fmt::Display for UnexpectedBeginOrEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected BEGIN or END")
    }
}

impl std::error::Error for UnexpectedBeginOrEnd {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
}
// TODO: how do I express <<alpha_or_dash, but not "END">>
pub fn property_key(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    if &input[0..=2] == "END" || &input[0..=4] == "BEGIN" {
        IResult::Err(Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Satisfy,
        )))
    } else {
        take_while(|c: char| (c == ',' || c == '/' || c == '_' || c == '-' || c.is_alphanumeric()))(
            input,
        )
    }
}

pub fn line<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    prefix: &'a str,
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    line_separated(complete(preceded(tag(prefix), f)))
}

pub fn line_separated<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    delimited(many0(line_ending), f, many0(line_ending))
}

pub fn unfold(input: &str) -> String {
    let mut output = Vec::<u8>::new();

    // unfold
    AhoCorasick::new(&["\n "])
        .stream_replace_all(input.as_bytes(), &mut output, &[""])
        .unwrap();

    String::from_utf8(output).unwrap()
}

pub fn simplify_line_endings(input: &str) -> String {
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

    assert_eq!(unfold(&simplify_line_endings(&input)), expected);
}
