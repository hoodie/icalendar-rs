use nom::{IResult, branch::alt, bytes::complete::{tag, take_till1}, character::complete::{alpha0, space0}, combinator::eof, error::VerboseError, multi::many0};
#[cfg(test)]
use pretty_assertions::assert_eq;

/// Zero-copy version of `properties::Parameter`
#[derive(PartialEq, Debug, Clone)]
pub struct Parameter<'a> {
    pub key: &'a str,
    pub val: &'a str,
}

impl<'a> Into<crate::properties::Parameter> for Parameter<'a> {
    fn into(self) -> crate::properties::Parameter {
        crate::properties::Parameter::new(self.key, self.val)
    }
}

#[test]
fn test_parameter() {
    assert_parser!(
        parameter(";KEY=VALUE"),
        Parameter {
            key: "KEY",
            val: "VALUE"
        }
    );
    assert_parser!(
        parameter("; KEY=VALUE"),
        Parameter {
            key: "KEY",
            val: "VALUE"
        }
    );
    assert_parser!(
        parameter("; KEY=VAL UE"),
        Parameter {
            key: "KEY",
            val: "VAL UE"
        }
    );
    assert_parser!(
        parameter("; KEY="),
        Parameter {
            key: "KEY",
            val: ""
        }
    );
    assert_parser!(
        parameter(";KEY=VAL-UE"),
        Parameter {
            key: "KEY",
            val: "VAL-UE"
        }
    );
}

#[test]
#[rustfmt::skip]
fn test_parameter_error() {
    assert!(parameter(";KEY").is_err());
}

fn parameter(i: &str) -> IResult<&str, Parameter, VerboseError<&str>> {
    let (i, _) = tag(";")(i)?;
    let (i, _) = space0(i)?;
    let (i, key) = alpha0(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, val) = alt((eof, take_till1(|x| x == ';' || x == ':')))(i)?;
    Ok((i, Parameter { key, val }))
}

// parameter list
#[test]
pub fn parse_parameter_list() {
    assert_parser!(
        parameters(";KEY=VALUE"),
        vec![Parameter {
            key: "KEY",
            val: "VALUE"
        }]
    );

    assert_parser!(
        parameters(";KEY=VALUE;DATE=TODAY"),
        vec![
            Parameter {
                key: "KEY",
                val: "VALUE"
            },
            Parameter {
                key: "DATE",
                val: "TODAY"
            }
        ]
    );

    assert_parser!(
        parameters(";KEY=VALUE;DATE=20170218"),
        vec![
            Parameter {
                key: "KEY",
                val: "VALUE"
            },
            Parameter {
                key: "DATE",
                val: "20170218"
            }
        ]
    );
}

pub fn parameters(i: &str) -> IResult<&str, Vec<Parameter>, VerboseError<&str>> {
    many0(parameter)(i)
}
