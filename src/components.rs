use chrono::*;
use uuid::Uuid;

// use std::io;
use std::collections::BTreeMap;
use std::fmt;
use std::mem;

use crate::properties::*;

/// Representation of various forms of `DATE-TIME` per
/// [RFC 5545, Section 3.3.5](https://tools.ietf.org/html/rfc5545#section-3.3.5)
///
/// Conversions from [chrono] types are provided in form of [From] implementations, see
/// documentation of individual variants.
///
/// In addition to readily implemented `FORM #1` and `FORM #2`, the RFC also specifies
/// `FORM #3: DATE WITH LOCAL TIME AND TIME ZONE REFERENCE`. This variant is not yet implemented.
/// Adding it will require adding support for `VTIMEZONE` and referencing it using `TZID`.
#[derive(Clone, Copy, Debug)]
pub enum CalendarDateTime {
    /// `FORM #1: DATE WITH LOCAL TIME`: floating, follows current time-zone of the attendee.
    ///
    /// Conversion from [`chrono::NaiveDateTime`] results in this variant.
    Floating(NaiveDateTime),
    /// `FORM #2: DATE WITH UTC TIME`: rendered with Z suffix character.
    ///
    /// Conversion from [`chrono::DateTime<Utc>`](DateTime) results in this variant. Use
    /// `date_time.with_timezone(&Utc)` to convert `date_time` from arbitrary time zone to UTC.
    Utc(DateTime<Utc>),
}

impl fmt::Display for CalendarDateTime {
    /// Format date-time in RFC 5545 compliant manner.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            CalendarDateTime::Floating(naive_dt) => naive_dt.format("%Y%m%dT%H%M%S").fmt(f),
            CalendarDateTime::Utc(utc_dt) => utc_dt.format("%Y%m%dT%H%M%SZ").fmt(f),
        }
    }
}

/// Converts from time zone-aware UTC date-time to [CalendarDateTime::Utc].
impl From<DateTime<Utc>> for CalendarDateTime {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::Utc(dt)
    }
}

/// Converts from time zone-less date-time to [CalendarDateTime::Floating].
impl From<NaiveDateTime> for CalendarDateTime {
    fn from(dt: NaiveDateTime) -> Self {
        Self::Floating(dt)
    }
}

/// VEVENT [(RFC 5545, Section 3.6.1 )](https://tools.ietf.org/html/rfc5545#section-3.6.1)
#[derive(Debug, Default)]
pub struct Event {
    inner: InnerComponent,
}

/// VTODO  [(RFC 5545, Section 3.6.2 )](https://tools.ietf.org/html/rfc5545#section-3.6.2)
#[derive(Debug, Default)]
pub struct Todo {
    inner: InnerComponent,
}

/// VVENUE  [(ical-venue)](https://tools.ietf.org/html/draft-norris-ical-venue-01)
#[derive(Debug, Default)]
pub struct Venue {
    inner: InnerComponent,
}

#[derive(Debug, Default)]
struct InnerComponent {
    properties: BTreeMap<String, Property>,
    multi_properties: Vec<Property>,
}

//impl<'a> Into<InnerComponent> for parse::Component<'a> {
//    fn into(self) -> InnerComponent {
//        unimplemented!()
//    }
//}


impl InnerComponent {
    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        InnerComponent {
            properties: mem::replace(&mut self.properties, BTreeMap::new()),
            multi_properties: mem::replace(&mut self.multi_properties, Vec::new()),
        }
    }
}

impl Event {
    /// Creates a new Event.
    pub fn new() -> Self {
        Default::default()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Event {
            inner: self.inner.done(),
        }
    }

    ///  Defines the overall status or confirmation
    pub fn status(&mut self, status: EventStatus) -> &mut Self {
        self.append_property(status.into());
        self
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}
}

impl Todo {
    /// Creates a new Todo.
    pub fn new() -> Self {
        Default::default()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Todo {
            inner: self.inner.done(),
        }
    }

    /// Set the PERCENT-COMPLETE `Property`
    ///
    /// Ranges between 0 - 100
    pub fn percent_complete(&mut self, percent: u8) -> &mut Self {
        self.add_property("PERCENT-COMPLETE", &percent.to_string());
        self
    }

    /// Set the DUE `Property`
    ///
    /// See [CalendarDateTime] for info how are different [chrono] types converted automatically.
    pub fn due<T: Into<CalendarDateTime>>(&mut self, dt: T) -> &mut Self {
        let calendar_dt: CalendarDateTime = dt.into();
        self.add_property("DUE", &calendar_dt.to_string());
        self
    }

    /// Set the COMPLETED `Property`
    ///
    /// Per [RFC 5545, Section 3.8.2.1](https://tools.ietf.org/html/rfc5545#section-3.8.2.1), this
    /// must be a date-time in UTC format.
    pub fn completed(&mut self, dt: DateTime<Utc>) -> &mut Self {
        self.add_property("COMPLETED", &CalendarDateTime::Utc(dt).to_string());
        self
    }

    ///  Defines the overall status or confirmation
    pub fn status(&mut self, status: TodoStatus) -> &mut Self {
        self.append_property(status.into());
        self
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}
}

impl Venue {
    /// Creates a new Venue.
    pub fn new() -> Self {
        Default::default()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Venue {
            inner: self.inner.done(),
        }
    }

    /// Set the STREET-ADDRESS `Property`
    ///
    /// This specifies the street address of a location. If the location requires a multiple-line
    /// address, they may be separated by an encoded newline "\n".
    pub fn street_address(&mut self, address: &str) -> &mut Self {
        self.add_property("STREET-ADDRESS", address)
    }

    /// Set the EXTENDED-ADDRESS `Property`
    ///
    /// This property provides the opportunity to include extended address information for a
    /// location. This property may be used to give additional information about an address that is
    /// not usually considered part of the street address. If the location requires a multiple-line
    /// address, they may be separated by an encoded newline "\n".
    pub fn extended_address(&mut self, address: &str) -> &mut Self {
        self.add_property("EXTENDED-ADDRESS", address)
    }

    /// Set the LOCALITY `Property`
    ///
    /// This specifies the city or locality of a venue.
    pub fn locality(&mut self, locality: &str) -> &mut Self {
        self.add_property("LOCALITY", locality)
    }

    /// Set the REGION `Property`
    ///
    /// This specifies the region (state, province, canton, etc.) of a location.
    pub fn region(&mut self, region: &str) -> &mut Self {
        self.add_property("REGION", region)
    }

    /// Set the COUNTRY `Property`
    ///
    /// This specifies the country of a location.
    pub fn country(&mut self, country: &str) -> &mut Self {
        self.add_property("COUNTRY", country)
    }

    /// Set the POSTAL-CODE `Property`
    ///
    /// This specifies the postal code of a location.
    pub fn postal_code(&mut self, postal_code: &str) -> &mut Self {
        self.add_property("POSTAL-CODE", postal_code)
    }
}

/// Implemented by everything that goes into a `Calendar`
pub trait Component {
    /// Returns kind of component.
    ///
    ///
    /// Must be ALL CAPS
    /// These are used in the `BEGIN` and `END` line of the component.
    fn component_kind() -> &'static str;

    /// Allows access to the inner properties map.
    fn properties(&self) -> &BTreeMap<String, Property>;

    /// Read-only access to `multi_properties`
    fn multi_properties(&self) -> &Vec<Property>;

    /// Writes `Component` into a `Writer` using `std::fmt`.
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        write_crlf!(out, "BEGIN:{}", Self::component_kind())?;

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

        write_crlf!(out, "END:{}", Self::component_kind())?;
        Ok(())
    }

    /// Guess what
    fn to_string(&self) -> String {
        let mut out_string = String::new();
        self.fmt_write(&mut out_string).unwrap();
        out_string
    }

    /// Append a given `Property`
    fn append_property(&mut self, property: Property) -> &mut Self;

    /// Adds a `Property` of which there may be many
    fn append_multi_property(&mut self, property: Property) -> &mut Self;

    /// Construct and append a `Property`
    fn add_property(&mut self, key: &str, val: &str) -> &mut Self {
        self.append_property(Property::new(key, val));
        self
    }

    /// Construct and append a `Property`
    fn add_multi_property(&mut self, key: &str, val: &str) -> &mut Self {
        self.append_multi_property(Property::new(key, val));
        self
    }

    /// Set the DTSTART `Property`
    ///
    /// See [CalendarDateTime] for info how are different [chrono] types converted automatically.
    fn starts<T: Into<CalendarDateTime>>(&mut self, dt: T) -> &mut Self {
        let calendar_dt = dt.into();
        self.add_property("DTSTART", &calendar_dt.to_string());
        self
    }

    /// Set the DTEND `Property`
    ///
    /// See [CalendarDateTime] for info how are different [chrono] types converted automatically.
    fn ends<T: Into<CalendarDateTime>>(&mut self, dt: T) -> &mut Self {
        let calendar_dt = dt.into();
        self.add_property("DTEND", &calendar_dt.to_string());
        self
    }

    /// Set the DTSTART `Property`, date only
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

    /// Set the DTEND `Property`, date only
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

    /// Set the DTSTART `Property`, date only
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
            /// Tells you what kind of `Component` this is
            ///
            /// Might be `VEVENT`, `VTODO`, `VALARM` etc
            fn component_kind() -> &'static str {
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

            /// Adds a `Property`
            fn append_property(&mut self, property: Property) -> &mut Self {
                self.inner
                    .properties
                    .insert(property.key().to_owned(), property);
                self
            }

            /// Adds a `Property` of which there may be many
            fn append_multi_property(&mut self, property: Property) -> &mut Self {
                self.inner.multi_properties.push(property);
                self
            }
        }
    };
}

component_impl! { Event, "VEVENT" }
component_impl! { Todo , "VTODO"}
component_impl! { Venue , "VVENUE"}
