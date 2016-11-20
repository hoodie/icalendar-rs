use chrono::*;
use uuid::Uuid;

// use std::io;
use std::fmt;
use std::mem;
use std::collections::HashMap;

use properties::*;

/// VEVENT [(RFC 5545, Section 3.6.1 )](https://tools.ietf.org/html/rfc5545#section-3.6.1)
#[derive(Debug, Default)]
pub struct Event { properties: HashMap<String,Property> }

/// VTODO  [(RFC 5545, Section 3.6.2 )](https://tools.ietf.org/html/rfc5545#section-3.6.2)
#[derive(Debug, Default)]
pub struct Todo { properties: HashMap<String,Property> }

impl Event {

    /// Creates a new Event.
    pub fn new() -> Self {
        Default::default()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Event { properties: mem::replace(&mut self.properties, HashMap::new()) }
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
        Todo { properties: mem::replace(&mut self.properties, HashMap::new()) }
    }

    //pub fn repeats<R:Repeater+?Sized>(&mut self, repeat: R) -> &mut Self {
    //    unimplemented!()
    //}
}


/// Implemented by everything that goes into a `Calendar`
pub trait Component {

    /// Returns kind of component.
    ///
    ///
    /// Must be ALL CAPS
    /// These are used in the `BEGIN` and `END` line of the component.
    fn component_kind() -> &'static str;

    /// Allows access to the inner properties HashMap.
    fn properties(&self) -> &HashMap<String,Property>;

    /// Writes `Component` into a `Writer` using `std::fmt`.
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {

        writeln!(out, "BEGIN:{}", Self::component_kind())?;
        let now = UTC::now().format("%Y%m%dT%H%M%S");
        writeln!(out, "DTSTAMP:{}", now)?;
        writeln!(out, "UID:{}", Uuid::new_v4())?;

        for property in self.properties().values() {
            property.fmt_write(out)?;
        }
        writeln!(out, "END:{}", Self::component_kind())?;
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

    /// Construct and append a `Property`
    fn add_property(&mut self, key: &str, val: &str) -> &mut Self {
        self.append_property( Property::new(key, val));
        self
    }


    /// Set the DTSTART `Property`
    fn starts<TZ:TimeZone>(&mut self, dt: DateTime<TZ>) -> &mut Self
        where TZ::Offset: fmt::Display
    {
        // DTSTART
        self.add_property("DTSTART", dt.format("%Y%m%dT%H%M%S").to_string().as_ref());
        self
    }

    /// Set the DTEND `Property`
    fn ends<TZ:TimeZone>(&mut self, dt: DateTime<TZ>) -> &mut Self
        where TZ::Offset: fmt::Display
    {
        // TODO don't manually use format but the rfc method, but test timezone behaviour
        self.add_property("DTEND", dt.format("%Y%m%dT%H%M%S").to_string().as_ref());
        self
    }

    /// Set the DTSTART `Property`, date only
    fn start_date<TZ:TimeZone>(&mut self, date: Date<TZ>) -> &mut Self
        where TZ::Offset: fmt::Display
    {
        // DTSTART
        self.append_property(
            Property::new("DTSTART", date.format("%Y%m%d").to_string().as_ref())
            .append_parameter(ValueType::Date)
            .done());
        self
    }

    /// Set the DTEND `Property`, date only
    fn end_date<TZ:TimeZone>(&mut self, date: Date<TZ>) -> &mut Self
        where TZ::Offset: fmt::Display
    {
        // DTSTART
        self.append_property(
            Property::new("DTEND", date.format("%Y%m%d").to_string().as_ref())
            .append_parameter(ValueType::Date)
            .done());
        self
    }

    /// Set the DTSTART `Property`, date only
    fn all_day<TZ:TimeZone>(&mut self, date: Date<TZ>) -> &mut Self
        where TZ::Offset: fmt::Display
    {
        // DTSTART
        self.append_property( Property::new("DTSTART", date.format("%Y%m%d").to_string().as_ref()).append_parameter(ValueType::Date).done() )
            .append_property( Property::new("DTEND",   date.format("%Y%m%d").to_string().as_ref()).append_parameter(ValueType::Date).done() )
            ;
        self
    }


    /// Prints to stdout
    fn print(&self) -> Result<(), fmt::Error> {
        let mut out = String::new();
        try!(self.fmt_write(&mut out));
        println!("{}", out);
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
                fn component_kind() -> &'static str { $kind }

                /// Read-only access to properties
                fn properties(&self) -> &HashMap<String, Property> {
                    &self.properties
                }

                /// Adds a `Property`
                fn append_property(&mut self, property: Property) -> &mut Self {
                    self.properties.insert(property.key(), property);
                    self
                }
            }
    }
}

component_impl! { Event, "VEVENT" }
component_impl! { Todo , "VTODO"}

