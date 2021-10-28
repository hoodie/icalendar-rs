use chrono::*;
use uuid::Uuid;

// use std::io;
use std::collections::BTreeMap;
use std::convert::Into;
use std::fmt;

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

/// Converts from time zone-aware UTC date-time to [`CalendarDateTime::Utc`].
impl From<DateTime<Utc>> for CalendarDateTime {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::Utc(dt)
    }
}

/// Converts from time zone-less date-time to [`CalendarDateTime::Floating`].
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
    properties: BTreeMap<String, Vec<Property>>,
}

impl Event {
    /// Creates a new Event.
    pub fn new() -> Self {
        Default::default()
    }

    ///  Defines the overall status or confirmation
    pub fn status(self, status: EventStatus) -> Self {
        self.with_property_set(status.into())
    }

    //pub fn repeats<R:Repeater+?Sized>(self, repeat: R) -> Self {
    //    unimplemented!()
    //}
}

impl Todo {
    /// Creates a new Todo.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the `PERCENT-COMPLETE` property
    ///
    /// Ranges between 0 - 100
    pub fn percent_complete(self, percent: u8) -> Self {
        self.property("PERCENT-COMPLETE", &percent.to_string())
    }

    /// Set the `DUE` property
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    pub fn due<T: Into<CalendarDateTime>>(self, dt: T) -> Self {
        let calendar_dt: CalendarDateTime = dt.into();
        self.property("DUE", &calendar_dt.to_string())
    }

    /// Set the `COMPLETED` property
    ///
    /// Per [RFC 5545, Section 3.8.2.1](https://tools.ietf.org/html/rfc5545#section-3.8.2.1), this
    /// must be a date-time in UTC format.
    pub fn completed(self, dt: DateTime<Utc>) -> Self {
        self.property("COMPLETED", &CalendarDateTime::Utc(dt).to_string())
    }

    ///  Defines the overall status or confirmation
    pub fn status(self, status: TodoStatus) -> Self {
        self.with_property_set(status.into())
    }

    //pub fn repeats<R:Repeater+?Sized>(self, repeat: R) -> Self {
    //    unimplemented!()
    //}
}

impl Venue {
    /// Creates a new Venue.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the STREET-ADDRESS `Property`
    ///
    /// This specifies the street address of a location. If the location requires a multiple-line
    /// address, they may be separated by an encoded newline "\n".
    pub fn street_address(self, address: &str) -> Self {
        self.property("STREET-ADDRESS", address)
    }

    /// Set the EXTENDED-ADDRESS `Property`
    ///
    /// This property provides the opportunity to include extended address information for a
    /// location. This property may be used to give additional information about an address that is
    /// not usually considered part of the street address. If the location requires a multiple-line
    /// address, they may be separated by an encoded newline "\n".
    pub fn extended_address(self, address: &str) -> Self {
        self.property("EXTENDED-ADDRESS", address)
    }

    /// Set the LOCALITY `Property`
    ///
    /// This specifies the city or locality of a venue.
    pub fn locality(self, locality: &str) -> Self {
        self.property("LOCALITY", locality)
    }

    /// Set the REGION `Property`
    ///
    /// This specifies the region (state, province, canton, etc.) of a location.
    pub fn region(self, region: &str) -> Self {
        self.property("REGION", region)
    }

    /// Set the COUNTRY `Property`
    ///
    /// This specifies the country of a location.
    pub fn country(self, country: &str) -> Self {
        self.property("COUNTRY", country)
    }

    /// Set the POSTAL-CODE `Property`
    ///
    /// This specifies the postal code of a location.
    pub fn postal_code(self, postal_code: &str) -> Self {
        self.property("POSTAL-CODE", postal_code)
    }
}

/// Implemented by everything that goes into a `Calendar`
pub trait Component: Sized {
    /// Returns kind of component.
    ///
    ///
    /// Must be ALL CAPS
    /// These are used in the `BEGIN` and `END` line of the component.
    fn component_kind() -> &'static str;

    /// Set a property with a key to a value
    fn property<K, V>(self, key: K, val: V) -> Self
    where
        K: ToString,
        V: Into<Property>,
    {
        self.with_property_set((key.to_string(), val.into().into_iter()))
    }

    /// Appends a value to the property with the key
    fn property_appended<K, V>(self, key: K, val: V) -> Self
    where
        K: ToString,
        V: Into<Property>,
    {
        self.with_property_appended((key.to_string(), val.into().into_iter()))
    }

    /// Set a property with a key to multiple values
    fn multi_property<K, I, V>(self, key: K, values: I) -> Self
    where
        K: ToString,
        I: IntoIterator<Item = V>,
        V: Into<Property>,
    {
        self.with_property_set((key.to_string(), values.into_iter().map(Into::into)))
    }

    /// Appends multiple values to the property with the key
    fn multi_property_appended<K, I, V>(self, key: K, values: I) -> Self
    where
        K: ToString,
        I: IntoIterator<Item = V>,
        V: Into<Property>,
    {
        self.with_property_appended((key.to_string(), values.into_iter().map(Into::into)))
    }

    /// Allows access to the inner properties map.
    fn properties(&self) -> &BTreeMap<String, Vec<Property>>;

    /// Writes `Component` into a `Writer` using `std::fmt`.
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        write_crlf!(out, "BEGIN:{}", Self::component_kind())?;

        if !self.properties().contains_key("DTSTAMP") {
            let now = CalendarDateTime::Utc(Utc::now());
            write_crlf!(out, "DTSTAMP:{}", now)?;
        }

        for (key, properties) in self.properties() {
            for property in properties {
                property.fmt_write(key, out)?;
            }
        }

        if !self.properties().contains_key("UID") {
            write_crlf!(out, "UID:{}", Uuid::new_v4())?;
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

    /// Sets a `Property` through `(Key, Value)` tuple
    fn with_property_set<I>(self, property: (String, I)) -> Self
    where
        I: IntoIterator<Item = Property>;

    /// Appends a `Property` through `(Key, Value)` tuple
    fn with_property_appended<I>(self, property: (String, I)) -> Self
    where
        I: IntoIterator<Item = Property>;

    /// Set the DTSTART `Property`
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    fn starts<T: Into<CalendarDateTime>>(self, dt: T) -> Self {
        let calendar_dt = dt.into();
        self.property("DTSTART", calendar_dt)
    }

    /// Set the DTEND `Property`
    ///
    /// See [`CalendarDateTime`] for info how are different [`chrono`] types converted automatically.
    fn ends<T: Into<CalendarDateTime>>(self, dt: T) -> Self {
        let calendar_dt = dt.into();
        self.property("DTEND", calendar_dt)
    }

    /// Set the DTSTART `Property`, date only
    fn start_date<TZ: TimeZone>(self, date: Date<TZ>) -> Self
    where
        TZ::Offset: fmt::Display,
    {
        // DTSTART
        self.property(
            "DTSTART",
            Property::from(date.format("%Y%m%d")).with_parameter_set(ValueType::Date),
        )
    }

    /// Set the `DTEND` property, date only
    fn end_date<TZ: TimeZone>(self, date: Date<TZ>) -> Self
    where
        TZ::Offset: fmt::Display,
    {
        // DTSTART
        self.property(
            "DTEND",
            Property::from(date.format("%Y%m%d")).with_parameter_set(ValueType::Date),
        )
    }

    /// Set the DTSTART `Property`, date only
    fn all_day<TZ: TimeZone>(self, date: Date<TZ>) -> Self
    where
        TZ::Offset: fmt::Display,
    {
        // DTSTART
        self.property(
            "DTSTART",
            Property::from(date.format("%Y%m%d")).with_parameter_set(ValueType::Date),
        )
        .property(
            "DTEND",
            Property::from(date.format("%Y%m%d")).with_parameter_set(ValueType::Date),
        )
    }

    ///  Defines the relative priority.
    ///
    ///  Ranges from 0 to 10, larger values will be truncated
    fn priority(self, priority: u32) -> Self {
        let priority = std::cmp::min(priority, 10);
        self.property("PRIORITY", &priority.to_string())
    }

    /// Prints to stdout
    fn print(&self) -> Result<(), fmt::Error> {
        let mut out = String::new();
        self.fmt_write(&mut out)?;
        print_crlf!("{}", out);
        Ok(())
    }

    /// Set the summary
    fn summary(self, desc: &str) -> Self {
        self.property("SUMMARY", desc)
    }

    /// Set the description
    fn description(self, desc: &str) -> Self {
        self.property("DESCRIPTION", desc)
    }

    ///// Set the description
    ///// TODO `Attendee` needs to be its own type
    //fn attendee( self, desc: &str) ->  Self {
    //    self.add_multi_property("ATTENDEE", desc) // multi_properties should be a multimap
    //}

    /// Set the LOCATION
    /// 3.8.1.7.  Location
    fn location(self, location: &str) -> Self {
        self.property("LOCATION", location)
    }

    /// Set the LOCATION with a VVENUE UID
    /// iCalender venue draft
    fn venue(self, location: &str, venue_uid: &str) -> Self {
        self.property(
            "LOCATION",
            Property::from(location).parameter("VVENUE", venue_uid),
        )
    }

    /// Set the UID
    fn uid(self, uid: &str) -> Self {
        self.property("UID", uid)
    }

    /// Set the visibility class
    fn class(self, class: Class) -> Self {
        self.with_property_set(class.into())
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
            fn properties(&self) -> &BTreeMap<String, Vec<Property>> {
                &self.inner.properties
            }

            /// Sets a given `Property`
            fn with_property_set<I>(mut self, property: (String, I)) -> Self
            where
                I: IntoIterator<Item = Property>,
            {
                self.inner
                    .properties
                    .insert(property.0, property.1.into_iter().collect());
                self
            }

            /// Appends a given `Property`
            fn with_property_appended<I>(mut self, property: (String, I)) -> Self
            where
                I: IntoIterator<Item = Property>,
            {
                if let Some(v) = self.inner.properties.get_mut(&property.0) {
                    v.extend(property.1.into_iter());
                    self
                } else {
                    self.with_property_set(property)
                }
            }
        }
    };
}

component_impl! { Event, "VEVENT" }
component_impl! { Todo , "VTODO"}
component_impl! { Venue , "VVENUE"}
