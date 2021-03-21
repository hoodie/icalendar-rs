use super::{
    parameters::{read_parameters, Parameter},
    utils,
    utils::alpha_or_dash,
};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0},
    combinator::{map, opt},
    error::context,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
#[cfg(test)]
use pretty_assertions::assert_eq;
use utils::ical_line;

/// Zero-copy version of `properties::Property`
#[derive(PartialEq, Debug, Clone)]
pub struct Property<'a> {
    pub key: &'a str,
    pub val: &'a str,
    pub params: Vec<Parameter<'a>>,
}

#[test]
fn test_property() {
    assert_eq!(
        property("KEY:VALUE\n"),
        Ok((
            "",
            Property {
                key: "KEY",
                val: "VALUE",
                params: vec![]
            }
        ))
    );

    assert_parser!(
        property("KEY1;foo=bar:VALUE\n"),
        Property {
            key: "KEY1",
            val: "VALUE",
            params: vec![Parameter {
                key: "foo",
                val: "bar"
            }]
        }
    );

    assert_parser!(
        property("KEY2;foo=bar:VALUE space separated\n"),
        Property {
            key: "KEY2",
            val: "VALUE space separated",
            params: vec![Parameter {
                key: "foo",
                val: "bar"
            }]
        }
    );

    assert_parser!(
        property("KEY2;foo=bar:important:VALUE\n"),
        Property {
            key: "KEY2",
            val: "important:VALUE",
            params: vec![Parameter {
                key: "foo",
                val: "bar"
            }]
        }
    );

    // TODO: newlines followed by spaces must be ignored
    assert_parser!(
        property("KEY3;foo=bar:VALUE\\n newline separated\n"),
        Property {
            key: "KEY3",
            val: "VALUE\\n newline separated",
            params: vec![Parameter {
                key: "foo",
                val: "bar"
            }]
        }
    );
}

#[test]
#[rustfmt::skip]
fn parse_property_with_breaks() {

    let sample_0 = "DESCRIPTION:Hey, I'm gonna have a party\\n BYOB: Bring your own beer.\\n Hendrik\\n\n";

    let expectation = Property {
        key: "DESCRIPTION",
        val: "Hey, I'm gonna have a party\\n BYOB: Bring your own beer.\\n Hendrik\\n",
        params: vec![]
    };

    assert_eq!(property(sample_0), Ok(("", expectation)));
}

pub fn property(input: &str) -> IResult<&str, Property> {
    map(
        tuple((
            separated_pair(
                tuple((
                    preceded(multispace0, alpha_or_dash), // key
                    read_parameters,                      // params
                )),
                context("fun", tag(":")), // separator
                ical_line,                // val TODO: replace this with something simpler!
            ),
            opt(line_ending),
        )),
        |(((key, params), val), _)| Property { key, val, params },
    )(input)
}
