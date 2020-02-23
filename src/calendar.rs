use crate::components::*;

use crate::{Parameter, Property};
use chrono::Duration;
use std::convert::Into;
use std::fmt;
use std::iter::FromIterator;
use std::ops::Deref;

#[derive(Debug)]
pub enum CalendarElement {
    Todo(Todo),
    Event(Event),
    Venue(Venue),
}

impl Into<CalendarElement> for Event {
    fn into(self) -> CalendarElement {
        CalendarElement::Event(self)
    }
}

impl Into<CalendarElement> for Todo {
    fn into(self) -> CalendarElement {
        CalendarElement::Todo(self)
    }
}

impl Into<CalendarElement> for Venue {
    fn into(self) -> CalendarElement {
        CalendarElement::Venue(self)
    }
}

impl CalendarElement {
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        match *self {
            CalendarElement::Todo(ref todo) => todo.fmt_write(out),
            CalendarElement::Event(ref event) => event.fmt_write(out),
            CalendarElement::Venue(ref venue) => venue.fmt_write(out),
        }
    }
}

/// Represents a calendar
///
/// You can `.add()` `Component`s to this.
#[derive(Default, Debug)]
pub struct Calendar {
    properties: Vec<Property>,
    components: Vec<CalendarElement>,
}

impl Calendar {
    /// Creates a new Calendar.
    pub fn new() -> Self {
        Default::default()
    }

    #[deprecated(note = "Use .push() instead")]
    #[doc(hidden)]
    pub fn add<T: Into<CalendarElement>>(&mut self, component: T) -> &mut Self {
        self.push(component)
    }

    /// Moves all the elements of other into Self, leaving other empty.
    pub fn append(&mut self, other: &mut Calendar) {
        self.components.append(&mut other.components);
    }

    /// Append a given `Property` to the `Calendar`
    pub fn append_property(&mut self, property: Property) -> &mut Self {
        self.properties.push(property);
        self
    }

    /// Extends this `Calendar` with the contends of another.
    pub fn extend<T, U>(&mut self, other: T)
    where
        T: IntoIterator<Item = U>,
        U: Into<CalendarElement>,
    {
        self.components.extend(other.into_iter().map(|x| x.into()));
    }

    /// Appends an element to the back of the `Calendar`.
    pub fn push<T: Into<CalendarElement>>(&mut self, component: T) -> &mut Self {
        self.components.push(component.into());
        self
    }

    /// Set the NAME and X-WR-CALNAME `Property`s
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.append_property(Property::new("NAME", name));
        self.append_property(Property::new("X-WR-CALNAME", name));
        self
    }

    /// Set the DESCRIPTION and X-WR-CALDESC `Property`s
    pub fn description(&mut self, description: &str) -> &mut Self {
        self.append_property(Property::new("DESCRIPTION", description));
        self.append_property(Property::new("X-WR-CALDESC", description));
        self
    }

    /// Set the TIMEZONE-ID and X-WR-TIMEZONE `Property`s
    pub fn timezone(&mut self, timezone: &str) -> &mut Self {
        self.append_property(Property::new("TIMEZONE-ID", timezone));
        self.append_property(Property::new("X-WR-TIMEZONE", timezone));
        self
    }

    /// Set the REFRESH-INTERVAL and X-PUBLISHED-TTL `Property`s
    pub fn ttl(&mut self, duration: &Duration) -> &mut Self {
        let duration_string = duration.to_string();
        self.append_property(
            Property::new("REFRESH-INTERVAL", duration_string.as_str())
                .append_parameter(Parameter::new("VALUE", "DURATION"))
                .done(),
        );
        self.append_property(Property::new("X-PUBLISHED-TTL", duration_string.as_str()));
        self
    }

    /// Writes `Calendar` into a `Writer` using `std::fmt`.
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        write_crlf!(out, "BEGIN:VCALENDAR")?;
        write_crlf!(out, "VERSION:2.0")?;
        write_crlf!(out, "PRODID:ICALENDAR-RS")?;
        write_crlf!(out, "CALSCALE:GREGORIAN")?;

        for property in &self.properties {
            property.fmt_write(out)?;
        }

        for component in &self.components {
            component.fmt_write(out)?;
        }
        write_crlf!(out, "END:VCALENDAR")?;
        Ok(())
    }

    /// Prints to stdout
    pub fn print(&self) -> Result<(), fmt::Error> {
        print_crlf!("{}", self);
        Ok(())
    }
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_write(f)
    }
}

impl Deref for Calendar {
    type Target = [CalendarElement];

    fn deref(&self) -> &[CalendarElement] {
        self.components.deref()
    }
}

impl<C: Into<CalendarElement>> FromIterator<C> for Calendar {
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        Calendar {
            components: iter.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendar_extend_components() {
        let mut calendar = Calendar::new();
        let components = vec![
            CalendarElement::Event(Event::new()),
            CalendarElement::Event(Event::new()),
        ];
        calendar.extend(components);
        assert_eq!(calendar.components.len(), 2);
    }

    #[test]
    fn calendar_extend_events() {
        let mut calendar = Calendar::new();
        let events = vec![Event::new(), Event::new()];
        calendar.extend(events);
        assert_eq!(calendar.components.len(), 2);
    }
}
