use components::*;

use std::fmt;
use std::ops::Deref;
use std::convert::Into;


#[derive(Debug)]
pub enum CalendarElement{
    Todo(Todo),
    Event(Event)
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

impl CalendarElement {
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        match *self {
            CalendarElement::Todo(ref todo)   => todo.fmt_write(out),
            CalendarElement::Event(ref event) => event.fmt_write(out)
        }
    }
}

/// Represents a calendar
///
/// You can `.add()` `Component`s to this.
#[derive(Default,Debug)]
pub struct Calendar {
    components: Vec<CalendarElement>
}

impl Calendar {

    /// Creates a new Calendar.
    pub fn new() -> Self {
        Default::default()
    }

    #[deprecated(note="Use .push() instead")]
    #[doc(hidden)]
    pub fn add<T:Into<CalendarElement>>(&mut self, component:T) -> &mut Self {
        self.push(component)
    }

    /// Moves all the elements of other into Self, leaving other empty.
    pub fn append(&mut self, other: &mut Calendar) {
        self.components.append(&mut other.components);
    }

    /// Extends this `Calendar` with the contends of another.
    pub fn extend<T>(&mut self, other: T)
        where T: IntoIterator<Item=CalendarElement>
    {
        self.components.extend(other);
    }

    /// Appends an element to the back of the `Calendar`.
    pub fn push<T:Into<CalendarElement>>(&mut self, component:T) -> &mut Self {
        self.components.push(component.into());
        self
    }

    /// Writes `Calendar` into a `Writer` using `std::fmt`.
    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        writeln!(out, "BEGIN:VCALENDAR")?;
        writeln!(out, "VERSION:2.0")?;
        writeln!(out, "PRODID:ICALENDAR-RS")?;
        writeln!(out, "CALSCALE:GREGORIAN")?;
        writeln!(out, "\n")?;

        for component in &self.components {
            component.fmt_write(out)?;
            write!(out, "\n")?;
        }
        writeln!(out, "END:VCALENDAR")?;
        Ok(())
    }

    /// Prints to stdout
    /// FIXME code repetition
    pub fn print(&self) -> Result<(), fmt::Error> {
        let mut out = String::new();
        try!(self.fmt_write(&mut out));
        println!("{}", out);
        Ok(())
    }
}

impl ToString for Calendar {
    /// # panics
    fn to_string(&self) -> String {
        let mut out_string = String::new();
        self.fmt_write(&mut out_string).unwrap();
        out_string
    }
}

impl Deref for Calendar {
    type Target = [CalendarElement];

    fn deref(&self) -> &[CalendarElement]{
        self.components.deref()
    }
}
