
use aho_corasick::AhoCorasick;
use nom::{IResult, bytes::complete::take_while};
#[cfg(test)]
use pretty_assertions::assert_eq;

pub fn alpha_or_dash(i: &str) -> IResult<&str, &str> {
    take_while(|c: char| (c == ',' || c == '/' || c == '_' || c == '-' || c.is_alphanumeric()))(i)
}

pub fn ical_line(input: &str) -> IResult<&str, &str> {
    ical_line_check(input, |_| true)
}

// TODO: this is unneccessary, but I haven't found a better parser yet that just eats everything
pub fn ical_line_check<F>(input: &str, check: F) -> IResult<&str, &str>
where
    F: Fn(u8) -> bool,
{
    for (i, c) in input.as_bytes().windows(2).enumerate() {
        // println!("{:?}", (i, c, input.len()));
        let remainder = &input[i..];
        let output = &input[..i];
        if let Some(&x) = c.get(0) {
            if !(check(x) || (x as char).is_whitespace() || x == b'\n') {
                // println!("check failed {:?}", c);
                return Ok((remainder, output));
            }
        }
        if c.get(0) == Some(&b'\n') && c.get(1) != Some(&b' ') {
            // println!("no space after break {:?}", c);
            let remainder = &input[i..];
            let output = &input[..i];
            return Ok((remainder, output));
        }
    }
    // literally a corner case
    if input.as_bytes().last() == Some(&b'\n') {
        // TODO: cut off `'\r'` as well
        let remainder = &input[input.len() - 1..];
        let output = &input[..input.len() - 1];
        return Ok((remainder, output));
    }
    Ok(("", input))
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
