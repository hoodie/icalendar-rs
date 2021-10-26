use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha0,
    combinator::{all_consuming, complete, cut, map},
    error::{context, ContextError, ParseError},
    multi::{many0, many_till},
    IResult,
};

#[cfg(test)]
use nom::error::ErrorKind;

#[cfg(test)]
use super::parameters::Parameter;
use super::{
    properties::property,
    utils::{line, line_separated},
    Property,
};

#[cfg(test)]
use pretty_assertions::assert_eq;

#[cfg(test)]
use crate::assert_parser;

/// The parsing equivalent of [`crate::components::Component`]
#[derive(PartialEq, Debug, Clone)]
pub struct Component<'a> {
    pub name: &'a str,
    pub properties: Vec<Property<'a>>,
    pub components: Vec<Component<'a>>,
}

#[test]
#[rustfmt::skip]
fn parse_empty_component1() {
    assert_eq!(
        component::<(_, ErrorKind)>("BEGIN:VEVENT\nEND:VEVENT\n"),
        Ok(("", Component{name: "VEVENT", properties: vec![], components: vec![] }))
    );

}

#[test]
#[rustfmt::skip]
fn parse_empty_component2() {
    assert_eq!(
        component::<(_, ErrorKind)>("BEGIN:VEVENT\n\nEND:VEVENT\n"),
        Ok(("", Component{name: "VEVENT", properties: vec![], components: vec![]})),
        "empty component with empty line");
}

#[test]
#[rustfmt::skip]
fn parse_component() {
    // let sample_0 = "BEGIN:VEVENT\nKEY;foo=bar:VALUE\nKEY;foo=bar;DATE=20170218:VALUE\nEND:VEVENT\n";
    let sample_1 = "BEGIN:VEVENT
KEY;foo=bar:VALUE
KEY;foo=bar;DATE=20170218:VALUE
END:VEVENT
";

    //assert_eq!(from_utf8(sample_0), from_utf8(sample_1));

    let expectation = Component{name: "VEVENT", properties: vec![
            Property{key: "KEY", val: "VALUE", params: vec![
                Parameter{key:"foo", val: Some("bar")},
            ]},
            Property{key: "KEY", val: "VALUE", params: vec![
                Parameter{key:"foo", val: Some("bar")},
                Parameter{key:"DATE", val: Some("20170218")},
            ]},
            ], components: vec![]};

    println!("expectation: {:#?}", expectation);
    println!("vs reality : {:#?}", component::<(_, ErrorKind)>(sample_1));

    assert_eq!(
        component::<(_, ErrorKind)>(sample_1).unwrap().1,
        expectation.clone());
}

enum ComponentChild<'a> {
    Property(Property<'a>),
    Component(Component<'a>),
}

pub fn component<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Component, E> {
    let (input, name) = line("BEGIN:", alpha0)(input)?;

    let (input, (properties, components)) = map(
        many_till(
            cut(context(
                "component",
                alt((
                    map(line_separated(component), ComponentChild::Component),
                    map(line_separated(property), ComponentChild::Property),
                )),
            )),
            line("END:", cut(context("MISMATCHING END", tag(name)))),
        ),
        |(body_elements, _)| {
            let mut properties = Vec::new();
            let mut components = Vec::new();
            for el in body_elements {
                match el {
                    ComponentChild::Component(c) => components.push(c),
                    ComponentChild::Property(p) => properties.push(p),
                }
            }
            (properties, components)
        },
    )(input)?;

    let (input, _) = many0(tag("\n"))(input)?;

    Ok((
        input,
        Component {
            name,
            properties,
            components,
        },
    ))
}

#[test]
fn test_components() {
    assert_parser!(
        component::<(_, ErrorKind)>("BEGIN:FOO\nEND:FOO"),
        Component {
            name: "FOO",
            properties: vec![],
            components: vec![]
        }
    );

    assert_parser!(
        component::<(_, ErrorKind)>("BEGIN:FOO\nFOO-PROP:important: spam €\nEND:FOO"),
        Component {
            name: "FOO",
            properties: vec![Property {
                key: "FOO-PROP",
                val: "important: spam €",
                params: vec![]
            }],
            components: vec![]
        }
    );

    assert_parser!(
        component::<(_, ErrorKind)>(
            "BEGIN:FOO\nUID:e1c97b31-38bb-4b72-b94f-463a12ef5239\nFOO-PROP:sp.am\nEND:FOO"
        ),
        Component {
            name: "FOO",
            properties: vec![
                Property {
                    key: "UID",
                    val: "e1c97b31-38bb-4b72-b94f-463a12ef5239",
                    params: vec![]
                },
                Property {
                    key: "FOO-PROP",
                    val: "sp.am",
                    params: vec![]
                },
            ],
            components: vec![]
        }
    );
    assert_parser!(
        component::<(_, ErrorKind)>(
            "BEGIN:FOO\nFOO-PROP:spam\nBEGIN:BAR\nBAR-PROP:spam\nEND:BAR\nEND:FOO"
        ),
        Component {
            name: "FOO",
            properties: vec![Property {
                key: "FOO-PROP",
                val: "spam",
                params: vec![]
            }],
            components: vec![Component {
                name: "BAR",
                properties: vec![Property {
                    key: "BAR-PROP",
                    val: "spam",
                    params: vec![]
                }],
                components: vec![]
            }]
        }
    );
}

#[test]
#[ignore]
fn test_faulty_component() {
    assert_parser!(
        component::<(_, ErrorKind)>("BEGIN:FOO\nEND:F0O"),
        Component {
            name: "FOO",
            properties: vec![],
            components: vec![]
        }
    );
}

pub fn components<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<Component>, E> {
    complete(many0(all_consuming(component)))(input)
}

// pub fn components<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
//     input: &'a str,
// ) -> IResult<&'a str, Vec<Component>, E> {
//     complete(many0(component))(input)
// }
