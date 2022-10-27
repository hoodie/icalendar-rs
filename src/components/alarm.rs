use std::collections::HashMap;

use super::*;

/// VALARM [(RFC 5545, Section 3.6.6 )](https://tools.ietf.org/html/rfc5545#section-3.6.6)
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Alarm {
    pub(crate) inner: InnerComponent,
    // pub(crate) action: Option<Action>,
}

impl Alarm {
    /// Creates a new Alarm with the given [`Trigger`]
    /// The [`TRIGGER`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.3) property is mandatory.
    /// see also [Alarm Trigger Relationship](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.14)
    pub fn with_trigger<T: Into<Trigger>>(trigger: T) -> Self {
        let trigger: Trigger = trigger.into();
        Alarm::default().append_property(trigger.into()).done()
    }

    /// add the [`ACTION`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1) property
    pub fn and_action(&mut self, action: Action) -> Self {
        // self.add_property("ACTION", action.as_str());
        self.append_property(action.into());
        self.done()
    }

    /// add the [`ACTION`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1) property
    pub fn action(&mut self, action: Action) -> &mut Self {
        // self.add_property("ACTION", action.as_str());
        self.append_property(action.into());
        self
    }

    /// add the [`DURATION`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.5) property
    /// TODO: add this to Event and Venue as well
    pub fn duration(&mut self, duration: Duration) -> &mut Self {
        // self.add_property("ACTION", action.as_str());
        self.append_property(duration.into());
        self
    }

    /// add the [`REPEAT`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.2) property
    pub fn repeat<T: Copy + Clone + Into<Repeat>>(&mut self, repeat_count: T) -> &mut Self {
        let repeat: Repeat = repeat_count.into();
        self.append_property(repeat.into());
        self
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Alarm {
            inner: self.inner.done(),
            // TODO: add default action = None
        }
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}
}

/// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
    Audio,
    /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
    Email,
    /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
    Display,
    // /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
    // IanaToken(String),
    // /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
    // XName{vendor: String, name: String},
    /// what ever else
    Other(String),
}

impl ToString for Action {
    /// convert the ACTION into its serialized representation
    fn to_string(&self) -> String {
        match self {
            Action::Audio => "AUDIO".into(),
            Action::Email => "EMAIL".into(),
            Action::Display => "DISPLAY".into(),
            Action::Other(other) => other.clone(),
        }
    }
}

impl From<Action> for Property {
    fn from(action: Action) -> Self {
        Property {
            key: String::from("ACTION"),
            val: action.to_string(),
            params: HashMap::new(),
        }
    }
}

/// [rfc5545#section-3.8.6.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.2)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Repeat(
    u32, // technically a signed integer according to spec
);

impl From<u32> for Repeat {
    fn from(count: u32) -> Self {
        Repeat(count)
    }
}

impl From<Repeat> for Property {
    fn from(r: Repeat) -> Self {
        Property::new_pre_alloc("REPEAT".into(), r.0.to_string())
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Related {
    Start,
    End,
}

/// [rfc5545#section-3.8.6.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.3)
/// This property specifies when an alarm will trigger.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Trigger {
    /// Duration in relation to either Start or End of the event
    Duration(Duration, Option<Related>),
    /// Absolute DateTime of the Trigger
    DateTime(DatePerhapsTime),
}

impl From<Duration> for Trigger {
    fn from(duration: Duration) -> Self {
        Trigger::Duration(duration, None)
    }
}

impl<T> From<T> for Trigger
where
    DatePerhapsTime: From<T>,
{
    fn from(dt: T) -> Self {
        Trigger::DateTime(DatePerhapsTime::from(dt))
    }
}

impl From<(Duration, Related)> for Trigger {
    fn from((duration, related): (Duration, Related)) -> Self {
        Trigger::Duration(duration, Some(related))
    }
}

impl From<Trigger> for Property {
    fn from(trigger: Trigger) -> Self {
        match trigger {
            Trigger::Duration(duration, _) => {
                Property::new_pre_alloc("TRIGGER".into(), duration.to_string())
            }

            Trigger::DateTime(dt) => dt.to_property("TRIGGER"),
        }
    }
}

#[test]
fn test_trigger() {
    let prop: Property = dbg!(Trigger::from(Duration::weeks(14)).into());
    let mut out = String::new();
    prop.fmt_write(&mut out).unwrap();
    dbg!(out);
}
