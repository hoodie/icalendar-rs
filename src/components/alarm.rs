#![allow(unused_variables)]
use std::{collections::HashMap, fmt::Debug, str::FromStr};

use self::properties::*;
use super::*;

/// VALARM [(RFC 5545, Section 3.6.6 )](https://tools.ietf.org/html/rfc5545#section-3.6.6)
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Alarm {
    pub(crate) inner: InnerComponent,
    // pub(crate) action: Option<Action>,
}

impl Alarm {
    /// Creates a new Audio-
    /// [Alarm Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.6)
    ///
    /// ## Definition
    /// 'action' and 'trigger' are both REQUIRED,
    ///  but MUST NOT occur more than once.
    ///
    /// `action / trigger /`
    ///
    ///  'duration' and 'repeat' are both OPTIONAL,
    ///  and MUST NOT occur more than once each;
    ///  but if one occurs, so MUST the other.
    ///
    /// `duration / repeat /`
    ///
    ///  The following is OPTIONAL,
    ///  but MUST NOT occur more than once.
    ///
    /// `attach /`
    ///
    ///  The following is OPTIONAL,
    ///  and MAY occur more than once.
    ///
    /// `x-prop / iana-prop`
    ///
    pub fn audio<T: Into<Trigger>>(trigger: T) -> Self {
        let trigger: Trigger = trigger.into();
        Alarm::default()
            .append_property(Action::Audio)
            .append_property(trigger)
            .done()
    }

    /// Creates a new Display-
    /// [Alarm Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.6)
    ///
    /// ## Definition
    ///  The following are REQUIRED,
    ///  but MUST NOT occur more than once.
    ///
    /// `action / description / trigger /`
    ///
    ///  'duration' and 'repeat' are both OPTIONAL,
    ///  and MUST NOT occur more than once each;
    ///  but if one occurs, so MUST the other.
    ///
    /// `duration / repeat /`
    ///
    ///  The following is OPTIONAL,
    ///  and MAY occur more than once.
    ///
    /// `x-prop / iana-prop`
    pub fn display(description: String, trigger: impl Into<Trigger>) -> Self {
        let trigger: Trigger = trigger.into();
        Alarm::default()
            .append_property(Action::Display)
            .append_property(trigger)
            .done()
    }

    /// Creates a new Email-
    /// [Alarm Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.6)
    ///
    /// ## Definition
    ///
    ///  The following are all REQUIRED,
    ///  but MUST NOT occur more than once.
    ///
    /// `action / description / trigger / summary /`
    ///
    ///  The following is REQUIRED,
    ///  and MAY occur more than once.
    ///
    /// `attendee /`
    ///
    ///  'duration' and 'repeat' are both OPTIONAL,
    ///  and MUST NOT occur more than once each;
    ///  but if one occurs, so MUST the other.
    ///
    /// `duration / repeat /`
    ///
    ///  The following are OPTIONAL,
    ///  and MAY occur more than once.
    ///
    /// `attach / x-prop / iana-prop`
    pub fn email(description: String, trigger: impl Into<Trigger>, summary: String) -> Self {
        let trigger: Trigger = trigger.into();
        Alarm::default()
            .append_property(Action::Email)
            .append_property(trigger)
            // .append_property(("DESCRIPTION", description))
            // .append_property(("SUMMARY", summary))
            .done()
    }

    /// Sets duration the
    /// [`REPEAT`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.2) and
    /// [`DURATION`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.5) property,
    /// which must not occur independent from one another
    pub fn duration_and_repeat<T: Copy + Clone + Into<Repeat>>(
        &mut self,
        duration: Duration,
        repeat_count: T,
    ) -> &mut Self {
        // self.add_property("ACTION", action.as_str());
        self.append_property(duration);

        let repeat: Repeat = repeat_count.into();
        self.append_property(repeat);
        self
    }

    /// Add the [`ATTACH`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.1) property
    /// TODO: might have to move to Component
    pub fn attach(&mut self, attachment: ()) -> &mut Self {
        todo!()
        // self.append_multi_property(todo!())
    }

    /// Returns the get action of this [`Alarm`].
    pub(crate) fn get_action(&self) -> Option<Action> {
        self.property_value("ACTION")
            .and_then(|p| Action::from_str(p).ok())
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

#[test]
fn test_audio() {
    let alarm = dbg!(Alarm::audio((Duration::minutes(15), Related::Start)).done());
    assert_eq!(alarm.get_action(), Some(Action::Audio));
    // assert_eq!(alarm.get_trigger(), Some(_));
    // alarm.trigger= duration
    // alarm.trigger.related = start
    // alarm.trigger.repeat = 0
    alarm.print();
}

#[test]
fn test_display() {
    todo!()
}
#[test]
fn test_email() {
    todo!()
}

pub mod properties {
    use super::*;

    /// [rfc5545#section-3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) enum Action {
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

    impl std::str::FromStr for Action {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "AUDIO" => Action::Audio,
                "EMAIL" => Action::Email,
                "DISPLAY" => Action::Display,
                other => Action::Other(other.into()),
            })
        }
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
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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

    /// Alarm Trigger Relationship[rfc5545#section-3.2.14](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.14)
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Related {
        /// the parameter value START will set the alarm to trigger off the start of the calendar component
        Start,
        /// the parameter value END will set the alarm to trigger off the end of the calendar component
        End,
    }

    impl From<Related> for Parameter {
        fn from(related: Related) -> Self {
            match related {
                Related::Start => Parameter::new("RELATED", "START"),
                Related::End => Parameter::new("RELATED", "END"),
            }
        }
    }

    #[test]
    fn test_repeat_default() {
        assert_eq!(
            Property::from(Repeat::default()).try_into(),
            Ok(String::from("REPEAT:0\r\n"))
        )
    }

    /// [rfc5545#section-3.8.6.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.3),
    /// see also [Alarm Trigger Relationship](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.14)
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
                Trigger::Duration(duration, Some(related)) => {
                    Property::new_pre_alloc("TRIGGER".into(), duration.to_string())
                        .append_parameter(related)
                        .done()
                }

                Trigger::Duration(duration, None) => {
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
}
