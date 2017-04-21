//! A library (far from anything) to generate icalendars
//! This implementation is still far from complete, I haven't even read the entire [spec](http://tools.ietf.org/html/rfc5545) yet.
//! Instead I implemented the parts I needed first.
//! More to come, contributions very welcome.
//!
//!
//! ## Structure
//! * `Calendar`s consist of `Components`
//! * `Component`s are e.g. `Event` or `Todo`
//! * `Component`s consist of `Property`s
//! * `Property`s may have `Parameter`s
//!
//! ```rust
//! # use chrono::*;
//! # use icalendar::*;
//! let event = Event::new()
//!     .summary("test event")
//!     .description("here I have something really important to do")
//!     .starts(Utc::now())
//!     .class(Class::Confidential)
//!     .ends(Utc::now() + Duration::days(1))
//!     .append_property(Property::new("TEST", "FOOBAR")
//!               .add_parameter("IMPORTANCE", "very")
//!               .add_parameter("DUE", "tomorrow")
//!               .done())
//!     .done();
//!
//! let bday = Event::new()
//!     .all_day(Utc.ymd(2016, 3, 15))
//!     .summary("My Birthday")
//!     .description(
//! r#"Hey, I'm gonna have a party
//! BYOB: Bring your own beer.
//! Hendrik"#
//! )
//!     .done();
//!
//! let todo = Todo::new().summary("Buy some milk").done();
//!
//!
//! let mut calendar = Calendar::new();
//! calendar.push(event);
//! calendar.push(todo);
//! calendar.push(bday);
//! ```
//!
//! ## Breaking API Changes in version 0.7.0
//!
//! - [Todo::due] and [Todo::completed] now take their date-time argument by value rather than by
//!   reference
//! - [Todo::completed] now requires its [chrono::DateTime] argument to have exactly [chrono::Utc]
//!   specified as its time zone as mandated by the RFC.
//! - [Component::starts], [Component::ends] and [Todo::due] now take newly introduced
//!   [CalendarDateTime] (through `Into<CalendarDateTime>` indirection). This allows callers to
//!   define time zone handling. Conversions from [`chrono::NaiveDateTime`] and
//!   [`chrono::DateTime<Utc>`](chrono::DateTime) are provided for ergonomics, the latter also restoring API
//!   compatibility in case of UTC date-times.

#![warn(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    // unused_qualifications,
    missing_debug_implementations
)]

macro_rules! print_crlf {
    () => (print!("\r\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\r\n"), $($arg)*));
}

macro_rules! write_crlf {
    ($dst:expr) => (
        write!($dst, "\r\n")
    );
    ($dst:expr, $fmt:expr) => (
        write!($dst, concat!($fmt, "\r\n"))
    );
    ($dst:expr, $fmt:expr, $($arg:tt)*) => (
        write!($dst, concat!($fmt, "\r\n"), $($arg)*)
    );
}

//pub mod period;
mod calendar;
mod components;
pub mod parse;
mod properties;

//pub mod repeats;
pub use crate::calendar::Calendar;
pub use crate::components::{CalendarDateTime, Component, Event, Todo, Venue};
pub use crate::properties::{Class, Parameter, Property, ValueType};
pub use crate::properties::{EventStatus, TodoStatus};

// TODO Calendar TimeZone VTIMEZONE STANDARD DAYLIGHT (see thunderbird exports)
