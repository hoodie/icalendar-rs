use super::*;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, space0},
    multi::many0,
    IResult,
};
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
#[rustfmt::skip]
fn test_parameter() {
    let dbg = |x| {println!("{:?}", x); x};
    assert_eq!(
        dbg(read_parameter(";KEY=VALUE")),
        Ok(("", Parameter{key: "KEY", val: "VALUE"})));
    assert_eq!(
        dbg(read_parameter("; KEY=VALUE")),
        Ok(("", Parameter{key: "KEY", val: "VALUE"})));
    assert_eq!(
        dbg(read_parameter("; KEY=VAL UE")),
        Ok(("", Parameter{key: "KEY", val: "VAL UE"})));
    assert_eq!(
        dbg(read_parameter("; KEY=")),
        Ok(("", Parameter{key: "KEY", val: ""})));
    assert_eq!(
        dbg(read_parameter(";KEY=VAL-UE")),
        Ok(("", Parameter{key: "KEY", val: "VAL-UE"})));
}

#[test]
#[rustfmt::skip]
fn test_parameter_error() {
    assert!(read_parameter(";KEY").is_err());
}

fn read_parameter(i: &str) -> IResult<&str, Parameter> {
    let (i, _) = tag(";")(i)?;
    let (i, _) = space0(i)?;
    let (i, key) = alpha0(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, val) = utils::ical_line_check(i, |x| x != b';' && x != b':')?;
    Ok((i, Parameter { key, val }))
}

// parameter list
#[test]
#[rustfmt::skip]
pub fn parse_parameter_list() {
    assert_eq!(
        read_parameters(";KEY=VALUE"),
        Ok( ("", vec![Parameter{key: "KEY", val: "VALUE"}])));

    assert_eq!(
        read_parameters(";KEY=VALUE;DATE=TODAY"),
        Ok( ("", vec![
            Parameter{key: "KEY", val: "VALUE"},
            Parameter{key: "DATE", val:"TODAY"}
        ])));

    assert_eq!(
        read_parameters(";KEY=VALUE;DATE=20170218"),
        Ok( ("", vec![
            Parameter{key: "KEY", val: "VALUE"},
            Parameter{key: "DATE", val:"20170218"}
        ])));
}

pub fn read_parameters(i: &str) -> IResult<&str, Vec<Parameter>> {
    many0(read_parameter)(i)
}
