use aho_corasick::AhoCorasick;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    combinator::complete,
    error::ParseError,
    multi::many0,
    sequence::{delimited, preceded},
    IResult, Parser,
};
#[cfg(test)]
use pretty_assertions::assert_eq;

pub fn alpha_or_dash(i: &str) -> IResult<&str, &str> {
    take_while(|c: char| (c == ',' || c == '/' || c == '_' || c == '-' || c.is_alphanumeric()))(i)
}

pub fn ical_line(input: &str) -> IResult<&str, &str> {
    take_until("\n")(input)
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
    delimited(many0(tag("\n")), f, many0(tag("\n")))
}

pub fn unfold(input: &str) -> String {
    let mut output = Vec::<u8>::new();

    // unfold
    AhoCorasick::new(&["\r\n "])
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

    assert_eq!(simplify_line_endings(&unfold(&input)), expected);
}
