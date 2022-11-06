#![allow(unused_variables)]
use std::{collections::HashMap, fmt::Debug, str::FromStr};

use self::properties::*;
use super::*;

/// VALARM [(RFC 5545, Section 3.6.6 )](https://tools.ietf.org/html/rfc5545#section-3.6.6)
#[derive(Debug, PartialEq, Eq)]
pub struct Alarm {
    pub(crate) inner: InnerComponent,
    // pub(crate) action: Option<Action>,
}

impl Alarm {
    /// You are not supposed to create an empty Alarm by yourself since
    /// this would allow leaving out certain fields that are required,
    /// hench creating incompliant Alarms.
    pub(self) fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }

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
    #[cfg(test)]
    pub(self) fn get_action(&self) -> Option<Action> {
        self.property_value("ACTION")
            .and_then(|p| Action::from_str(p).ok())
    }

    /// Returns the get action of this [`Alarm`].
    #[cfg(test)]
    pub(self) fn get_trigger(&self) -> Option<Trigger> {
        self.inner
            .properties
            .get("TRIGGER")
            .and_then(|prop| Trigger::try_from(prop).ok())
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
    assert_eq!(
        alarm.get_trigger(),
        Some(Trigger::Duration(
            Duration::minutes(15),
            Related::Start.into()
        ))
    );
    // assert_eq!(alarm.get_trigger(), Some(_));
    // alarm.trigger= duration
    // alarm.trigger.related = start
    // alarm.trigger.repeat = 0
    alarm.print().unwrap();
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

    use crate::components::{alarm::properties::Parameter, date_time::parse_duration};

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

    impl FromStr for Action {
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

    impl FromStr for Repeat {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            todo!()
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

    impl FromStr for Related {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "START" => Ok(Related::Start),
                "END" => Ok(Related::End),
                _ => Err(()),
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
        DateTime(CalendarDateTime),
    }

    impl From<Duration> for Trigger {
        fn from(duration: Duration) -> Self {
            Trigger::Duration(duration, None)
        }
    }

    impl FromStr for Trigger {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            dbg!(s);
            todo!()
        }
    }

    impl<T> From<T> for Trigger
    where
        CalendarDateTime: From<T>,
    {
        fn from(dt: T) -> Self {
            Trigger::DateTime(CalendarDateTime::from(dt))
        }
    }

    impl From<(Duration, Related)> for Trigger {
        fn from((duration, related): (Duration, Related)) -> Self {
            Trigger::Duration(duration, Some(related))
        }
    }

    impl TryFrom<&Property> for Trigger {
        type Error = ();
        fn try_from(prop: &Property) -> Result<Self, Self::Error> {
            match (prop.key(), prop.params().get("VALUE").map(Parameter::value)) {
                ("TRIGGER", Some("DATE-TIME")) => {
                    // date-time needs to be qualified "VALUE=DATE-TIME"
                    if let Some(dt) = parse_utc_date_time(prop.value()) {
                        Ok(Trigger::from(dt))
                    } else {
                        Err(())
                    }
                }
                ("TRIGGER", Some("DURATION") | None) => {
                    // duration is the assumed default or "VALUE=DURATION"
                    let param_related = prop.get_param_as("RELATED", |s| Related::from_str(s).ok());

                    // TODO: improve error handling here
                    // TODO: yes I found icalendar-duration, let's find a way to integrate this if possible
                    let parsed_duration = prop.get_value_as(parse_duration);

                    if let Some(duration) = parsed_duration {
                        Ok(Trigger::Duration(duration, param_related))
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            }
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

                Trigger::DateTime(dt) => dt
                    .to_property("TRIGGER")
                    .add_parameter("VALUE", "DATE-TIME")
                    .done(),
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

    /// adding triggers to Alarm component serializes them and so we need to re-parse to read them
    /// I know this sucks, but let's do the refactoring of the internal representation elsewhere
    #[test]
    fn test_trigger_abs_from_str() {
        let now: CalendarDateTime = Utc::now().into();

        let alarm_with_abs_trigger = Alarm::default()
            .append_property(Trigger::from(now.clone()))
            .done();

        alarm_with_abs_trigger.print().unwrap();

        assert_eq!(
            alarm_with_abs_trigger.get_trigger(),
            Some(Trigger::DateTime(now))
        );
    }
    
    #[test]
    fn test_trigger_dur_from_str() {
        let dur = Duration::minutes(15);

        let alarm_with_rel_trigger = Alarm::default()
            .append_property(Trigger::from((Duration::minutes(15), Related::Start)))
            .done();
        let alarm_with_rel_start_trigger = Alarm::default()
            .append_property(Trigger::from((dur, Related::Start)))
            .done();

        assert_eq!(
            alarm_with_rel_trigger.get_trigger(),
            Some(Trigger::Duration(dur, None))
        );
        assert_eq!(
            alarm_with_rel_start_trigger.get_trigger(),
            Some(Trigger::Duration(dur, Some(Related::Start)))
        );
    }
}
