use chrono::*;
use uuid::Uuid;

use std::{collections::BTreeMap, fmt, mem};

use crate::properties::*;

mod date_time;
mod event;
mod other;
mod todo;
mod venue;

pub use date_time::CalendarDateTime;
pub use event::*;
pub use other::*;
pub use todo::*;
pub use venue::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Clone))]
pub(crate) struct InnerComponent {
    pub properties: BTreeMap<String, Property>,
    pub multi_properties: Vec<Property>,
}

//impl<'a> Into<InnerComponent> for parser::Component<'a> {
//    fn into(self) -> InnerComponent {
//        unimplemented!()
//    }
//}

impl InnerComponent {
    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        InnerComponent {
            properties: mem::take(&mut self.properties),
            multi_properties: mem::take(&mut self.multi_properties),
        }
    }
}

/// Implemented by everything that goes into a `Calendar`
pub trait Component {
    /// Returns kind of component.
    ///
    ///
    /// Must be ALL CAPS
    /// These are used in the `BEGIN` and `END` line of the component.
    fn component_kind(&self) -> String;

    /// Allows access to the inner properties map.
    fn properties(&self) -> &BTreeMap<String, Property>;

    /// Read-only access to `multi_properties`
    fn multi_properties(&self) -> &Vec<Property>;

    /// Writes [`Component`] using [`std::fmt`].
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        write_crlf!(out, "BEGIN:{}", self.component_kind())?;

        if !self.properties().contains_key("DTSTAMP") {
            let now = CalendarDateTime::Utc(Utc::now());
            write_crlf!(out, "DTSTAMP:{}", now)?;
        }

        for property in self.properties().values() {
            property.fmt_write(out)?;
        }

        if !self.properties().contains_key("UID") {
            write_crlf!(out, "UID:{}", Uuid::new_v4())?;
        }

        for property in self.multi_properties() {
            property.fmt_write(out)?;
        }

        write_crlf!(out, "END:{}", self.component_kind())?;
        Ok(())
    }

    /// Serializes this component into [`rfc5545`](http://tools.ietf.org/html/rfc5545) again
    ///
    /// # Panic
    /// this can panic if [`std::fmt::write`] returns an Error
    /// use [`Component::try_into_string()`] if you don't like panicking
    fn to_string(&self) -> String {
        self.try_into_string().unwrap()
    }

    /// Serializes this component into [`rfc5545`](http://tools.ietf.org/html/rfc5545) again
    fn try_into_string(&self) -> Result<String, std::fmt::Error> {
        let mut out_string = String::new();
        self.fmt_write(&mut out_string)?;
        Ok(out_string)
    }

    /// Append a given [`Property`]
    fn append_property(&mut self, property: Property) -> &mut Self;

    /// Adds a [`Property`] of which there may be many
    fn append_multi_property(&mut self, property: Property) -> &mut Self;

    /// Construct and append a [`Property`]
    fn add_property(&mut self, key: &str, val: &str) -> &mut Self {
        self.append_property(Property::new(key, val));
        self
    }

    /// Construct and append a [`Property`]
    fn add_multi_property(&mut self, key: &str, val: &str) -> &mut Self {
        self.append_multi_property(Property::new(key, val));
        self
    }

    /// Set the [`DTSTAMP`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.2) [`Property`]
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    fn timestamp<T: Into<CalendarDateTime>>(&mut self, dt: T) -> &mut Self {
        let calendar_dt = dt.into();
        self.add_property("DTSTAMP", &calendar_dt.to_string());
        self
    }

    /// Set the [`DTSTART`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4) [`Property`]
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    fn starts<T: Into<CalendarDateTime>>(&mut self, dt: T) -> &mut Self {
        let calendar_dt = dt.into();
        self.add_property("DTSTART", &calendar_dt.to_string());
        self
    }

    /// Set the [`DTEND`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2) [`Property`]
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    fn ends<T: Into<CalendarDateTime>>(&mut self, dt: T) -> &mut Self {
        let calendar_dt = dt.into();
        self.add_property("DTEND", &calendar_dt.to_string());
        self
    }

    /// Set the [`DTSTART`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4) [`Property`], date only
    fn start_date<TZ: TimeZone>(&mut self, date: Date<TZ>) -> &mut Self
    where
        TZ::Offset: fmt::Display,
    {
        // DTSTART
        self.append_property(
            Property::new("DTSTART", date.format("%Y%m%d").to_string().as_ref())
                .append_parameter(ValueType::Date)
                .done(),
        );
        self
    }

    /// Set the [`DTEND`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2) [`Property`], date only
    fn end_date<TZ: TimeZone>(&mut self, date: Date<TZ>) -> &mut Self
    where
        TZ::Offset: fmt::Display,
    {
        // DTSTART
        self.append_property(
            Property::new("DTEND", date.format("%Y%m%d").to_string().as_ref())
                .append_parameter(ValueType::Date)
                .done(),
        );
        self
    }

    /// Set the [`DTSTART`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4) [`Property`]
    /// and [`DTEND`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2) [`Property`],
    /// date only
    fn all_day<TZ: TimeZone>(&mut self, date: Date<TZ>) -> &mut Self
    where
        TZ::Offset: fmt::Display,
    {
        // DTSTART
        self.append_property(
            Property::new("DTSTART", date.format("%Y%m%d").to_string().as_ref())
                .append_parameter(ValueType::Date)
                .done(),
        )
        .append_property(
            Property::new("DTEND", date.format("%Y%m%d").to_string().as_ref())
                .append_parameter(ValueType::Date)
                .done(),
        );
        self
    }

    ///  Defines the relative priority.
    ///
    ///  Ranges from 0 to 10, larger values will be truncated
    fn priority(&mut self, priority: u32) -> &mut Self {
        let priority = ::std::cmp::min(priority, 10);
        self.add_property("PRIORITY", &priority.to_string());
        self
    }

    /// Prints to stdout
    fn print(&self) -> Result<(), fmt::Error> {
        let mut out = String::new();
        self.fmt_write(&mut out)?;
        print_crlf!("{}", out);
        Ok(())
    }

    /// Set the summary
    fn summary(&mut self, desc: &str) -> &mut Self {
        self.add_property("SUMMARY", desc)
    }

    /// Set the description
    fn description(&mut self, desc: &str) -> &mut Self {
        self.add_property("DESCRIPTION", desc)
    }

    ///// Set the description
    ///// TODO `Attendee` needs to be its own type
    //fn attendee(&mut self, desc: &str) -> &mut Self {
    //    self.add_multi_property("ATTENDEE", desc) // multi_properties should be a multimap
    //}

    /// Set the LOCATION
    /// 3.8.1.7.  Location
    fn location(&mut self, location: &str) -> &mut Self {
        self.add_property("LOCATION", location);
        self
    }

    /// Set the LOCATION with a VVENUE UID
    /// iCalender venue draft
    fn venue(&mut self, location: &str, venue_uid: &str) -> &mut Self {
        self.append_property(
            Property::new("LOCATION", location)
                .append_parameter(Parameter::new("VVENUE", venue_uid))
                .done(),
        );
        self
    }

    /// Set the UID
    fn uid(&mut self, uid: &str) -> &mut Self {
        self.add_property("UID", uid);
        self
    }

    /// Set the visibility class
    fn class(&mut self, class: Class) -> &mut Self {
        self.append_property(class.into())
    }
}

macro_rules! component_impl {
    ($t:ty, $kind:expr) => {
        impl Component for $t {
            /// Tells you what kind of [`Component`] this is
            ///
            /// Might be [`VEVENT`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1), [`VTODO`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.2), [`VALARM`](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.6) etc
            fn component_kind(&self) -> String {
                $kind
            }

            /// Read-only access to `properties`
            fn properties(&self) -> &BTreeMap<String, Property> {
                &self.inner.properties
            }

            /// Read-only access to `multi_properties`
            fn multi_properties(&self) -> &Vec<Property> {
                &self.inner.multi_properties
            }

            /// Adds a [`Property`]
            fn append_property(&mut self, property: Property) -> &mut Self {
                self.inner
                    .properties
                    .insert(property.key().to_owned(), property);
                self
            }

            /// Adds a [`Property`] of which there may be many
            fn append_multi_property(&mut self, property: Property) -> &mut Self {
                self.inner.multi_properties.push(property);
                self
            }
        }

        impl From<InnerComponent> for $t {
            fn from(inner: InnerComponent) -> $t {
                Self { inner }
            }
        }
    };
}

component_impl! { Event, String::from("VEVENT") }
component_impl! { Todo , String::from("VTODO")}
component_impl! { Venue , String::from("VVENUE")}
