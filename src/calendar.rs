use components::*;

use std::fmt;
use std::convert::Into;


pub enum CalendarElement{
    Todo(Todo),
    Event(Event)
}

impl Into<CalendarElement> for Event{
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
pub struct Calendar {
    components: Vec<CalendarElement>
}

impl Calendar {

    pub fn new() -> Self {
        Calendar {
            components: Vec::new()
        }
    }

    pub fn add<T:Into<CalendarElement>>(&mut self, component:T) -> &mut Self {
        self.components.push(component.into());
        self
    }

    fn fmt_write<W: fmt::Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        writeln!(out, "BEGIN:VCALENDAR")?;
        writeln!(out, "VERSION:2.0")?;
        writeln!(out, "\n")?;

        for component in &self.components {
            component.fmt_write(out)?;
            writeln!(out, "\n")?;
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
    fn to_string(&self) -> String {
        let mut out_string = String::new();
        self.fmt_write(&mut out_string).unwrap();
        out_string
    }
}
