use std::{fmt, str::FromStr};

use chrono::Utc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, complete, cut, map},
    error::{context, convert_error, ContextError, ParseError, VerboseError},
    multi::{many0, many_till},
    Finish, IResult,
};

#[cfg(test)]
use nom::error::ErrorKind;
use uuid::Uuid;

#[cfg(test)]
use super::parameters::Parameter;
use super::{
    properties::property,
    unfold,
    utils::{line, line_separated, valid_key_sequence},
    Property,
};

#[cfg(test)]
use pretty_assertions::assert_eq;

use crate::{
    calendar::CalendarComponent,
    components::{InnerComponent, Other},
    CalendarDateTime,
};

/// The parsing equivalent of [`crate::components::Component`]
#[derive(PartialEq, Debug, Clone)]
pub struct Component<'a> {
    pub name: &'a str,
    pub properties: Vec<Property<'a>>,
    pub components: Vec<Component<'a>>,
}

impl Component<'_> {
    /// Writes `Component` into a `Writer` using `std::fmt`.
    pub(crate) fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        write_crlf!(out, "BEGIN:{}", self.name)?;

        if self.name.to_lowercase() == "calendar" {
            if !self
                .properties
                .iter()
                .any(|property| property.key == "DTSTAMP")
            {
                let now = CalendarDateTime::Utc(Utc::now());
                write_crlf!(out, "DTSTAMP:{}", now)?;
            }

            if !self.properties.iter().any(|property| property.key == "UID") {
                write_crlf!(out, "UID:{}", Uuid::new_v4())?;
            }
        }
        for property in &self.properties {
            property.fmt_write(out)?;
        }

        for component in &self.components {
            component.fmt_write(out)?;
        }

        write_crlf!(out, "END:{}", self.name)?;
        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for Component<'a> {
    type Error = String;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        component(input)
            .finish()
            .map(|(_, x)| x)
            .map_err(|e: VerboseError<&str>| format!("error: {}", convert_error(input, e.clone())))
    }
}

impl From<Component<'_>> for InnerComponent {
    fn from(component: Component) -> Self {
        Self {
            properties: component
                .properties
                .into_iter()
                .map(|p| (p.key.into(), p.into()))
                .collect(),
            multi_properties: Default::default(),
        }
    }
}

impl<'a> From<Component<'a>> for CalendarComponent {
    fn from(component: Component<'_>) -> CalendarComponent {
        use crate::{Event, Todo, Venue};
        match component.name {
            "VEVENT" => Event::from(InnerComponent::from(component)).into(),
            "VTODO" => Todo::from(InnerComponent::from(component)).into(),
            "VVENUE" => Venue::from(InnerComponent::from(component)).into(),
            _ => Other::from((component.name.into(), InnerComponent::from(component))).into(),
        }
    }
}

impl<'a> FromStr for CalendarComponent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from_parsed = crate::CalendarComponent::from(read_component(&unfold(s))?);
        Ok(from_parsed)
    }
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
fn parse_empty_component_with_dash() {
    assert_eq!(
        component::<(_, ErrorKind)>("BEGIN:X-HOODIE-EVENT\n\nEND:X-HOODIE-EVENT\n"),
        Ok(("", Component{name: "X-HOODIE-EVENT", properties: vec![], components: vec![]})),
        "empty component with empty line");
}

#[test]
#[rustfmt::skip]
fn parse_component() {
    let sample_1 = "BEGIN:VEVENT
KEY;foo=bar:VALUE
KEY;foo=bar;DATE=20170218:VALUE
END:VEVENT
";

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
        expectation);
}

enum ComponentChild<'a> {
    Property(Property<'a>),
    Component(Component<'a>),
}

pub fn read_component(input: &str) -> Result<Component<'_>, String> {
    component(input)
        .finish()
        .map(|(_, component)| component)
        .map_err(|e: VerboseError<&str>| format!("error: {}", convert_error(input, e.clone())))
}

pub fn component<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Component, E> {
    let (input, name) = line("BEGIN:", valid_key_sequence)(input)?;

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
        component,
        "BEGIN:FOO\nEND:FOO",
        Component {
            name: "FOO",
            properties: vec![],
            components: vec![]
        }
    );

    assert_parser!(
        component,
        "BEGIN:FOO\nFOO-PROP:important: spam €\nEND:FOO",
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
        component,
        "BEGIN:FOO\nUID:e1c97b31-38bb-4b72-b94f-463a12ef5239\nFOO-PROP:sp.am\nEND:FOO",
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
        component,
        "BEGIN:FOO\nFOO-PROP:spam\nBEGIN:BAR\nBAR-PROP:spam\nEND:BAR\nEND:FOO",
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
fn test_nested_components() {
    assert_parser!(
        component,
        "BEGIN:FOO\nFOO-PROP:spam\nBEGIN:BAR\nBAR-PROP:spam\nBEGIN:BAR\nBAR-PROP:spam\nEND:BAR\nEND:BAR\nEND:FOO",
        Component {
            name: "FOO",
            properties: vec![Property {
                key: "FOO-PROP",
                val: "spam",
                params: vec![]
            }],
            components: vec![
                Component {
                    name: "BAR",
                    properties: vec![Property {
                        key: "BAR-PROP",
                        val: "spam",
                        params: vec![]
                    }],
                    components: vec![
                        Component {
                            name: "BAR",
                            properties: vec![Property {
                                key: "BAR-PROP",
                                val: "spam",
                                params: vec![]
                            }],
                            components: vec![]
                        },

                    ]
                },
            ]
        }
    );
}

#[test]
fn test_multi_components() {
    assert_parser!(
        component,
        r#"
BEGIN:VEVENT
BEGIN:VALARM
RELATED-TO;RELTYPE=:c605e4e8-8ea3-4315-b139-19394ab3ced6
END:VALARM
END:VEVENT
"#,
        Component {
            name: "VEVENT",
            properties: vec![],
            components: vec![Component {
                name: "VALARM",
                properties: vec![Property {
                    key: "RELATED-TO",
                    val: "c605e4e8-8ea3-4315-b139-19394ab3ced6",
                    params: vec![Parameter {
                        key: "RELTYPE",
                        val: None,
                    },],
                },],
                components: vec![],
            },],
        }
    );
    assert_parser!(
        component,
        "BEGIN:FOO\nFOO-PROP:spam\nBEGIN:BAR\nBAR-PROP:spam\nEND:BAR\nBEGIN:BAR\nBAR-PROP:spam\nEND:BAR\nEND:FOO",
        Component {
            name: "FOO",
            properties: vec![Property {
                key: "FOO-PROP",
                val: "spam",
                params: vec![]
            }],
            components: vec![
                Component {
                name: "BAR",
                properties: vec![Property {
                    key: "BAR-PROP",
                    val: "spam",
                    params: vec![]
                }],
                components: vec![]
            },
                Component {
                name: "BAR",
                properties: vec![Property {
                    key: "BAR-PROP",
                    val: "spam",
                    params: vec![]
                }],
                components: vec![]
            }
            ]
        }
    );
}

#[test]
#[should_panic]
fn test_faulty_component() {
    assert_parser!(
        component,
        "BEGIN:FOO\nEND:F0O",
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
