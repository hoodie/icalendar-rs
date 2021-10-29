use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{alpha0, space0},
    combinator::{eof, opt},
    error::{ContextError, ParseError},
    multi::many0,
    sequence::preceded,
    IResult,
};

#[cfg(test)]
use nom::error::ErrorKind;

/// Zero-copy version of [`crate::properties::Parameter`]
#[derive(PartialEq, Debug, Clone)]
pub struct Parameter<'a> {
    pub key: &'a str,
    pub val: Option<&'a str>,
}

impl<'a> From<Parameter<'a>> for crate::properties::Parameter {
    fn from(parameter: Parameter<'_>) -> crate::properties::Parameter {
        crate::properties::Parameter::new(parameter.key, parameter.val.unwrap_or(""))
    }
}

#[test]
fn test_parameter() {
    assert_parser!(
        parameter,
        ";KEY=VALUE",
        Parameter {
            key: "KEY",
            val: Some("VALUE")
        }
    );
    assert_parser!(
        parameter,
        "; KEY=VALUE",
        Parameter {
            key: "KEY",
            val: Some("VALUE")
        }
    );
    assert_parser!(
        parameter,
        "; KEY=VAL UE",
        Parameter {
            key: "KEY",
            val: Some("VAL UE")
        }
    );
    assert_parser!(
        parameter,
        "; KEY=",
        Parameter {
            key: "KEY",
            val: Some("")
        }
    );
    assert_parser!(
        parameter,
        ";KEY=VAL-UE",
        Parameter {
            key: "KEY",
            val: Some("VAL-UE")
        }
    );
    assert_parser!(
        parameter,
        ";KEY",
        Parameter {
            key: "KEY",
            val: None,
        }
    );

    assert_parser!(
        parameter,
        ";email=rust@hoodie.de",
        Parameter {
            key: "email",
            val: Some("rust@hoodie.de")
        }
    );
}

fn parameter<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Parameter, E> {
    let (i, _) = tag(";")(i)?;
    let (i, _) = space0(i)?;
    let (i, key) = alpha0(i)?;
    let (i, val) = opt(preceded(
        tag("="),
        alt((eof, take_till1(|x| x == ';' || x == ':'))),
    ))(i)?;
    Ok((i, Parameter { key, val }))
}

// parameter list
#[test]
pub fn parse_parameter_list() {
    assert_parser!(
        parameters,
        ";KEY=VALUE",
        vec![Parameter {
            key: "KEY",
            val: Some("VALUE")
        }]
    );

    assert_parser!(
        parameters,
        ";KEY=VALUE;DATE=TODAY",
        vec![
            Parameter {
                key: "KEY",
                val: Some("VALUE")
            },
            Parameter {
                key: "DATE",
                val: Some("TODAY")
            }
        ]
    );

    assert_parser!(
        parameters,
        ";KEY=VALUE;DATE=20170218",
        vec![
            Parameter {
                key: "KEY",
                val: Some("VALUE")
            },
            Parameter {
                key: "DATE",
                val: Some("20170218")
            }
        ]
    );
}

pub fn parameters<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<Parameter>, E> {
    many0(parameter)(input)
}
