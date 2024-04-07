//! # A library to generate and parse [iCalendars](http://tools.ietf.org/html/rfc5545).
//!
//! Contributions are very welcome.
//!
//!
//! ## Structure
//! * [`Calendar`]s consist of [`Component`]s
//! * [`Component`]s are e.g. [`Event`] or [`Todo`]
//! * [`Component`]s consist of [`Property`]s
//! * [`Property`]s may have [`Parameter`]s
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
//!     .all_day(NaiveDate::from_ymd(2023, 3, 15))
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
//! - [`Todo::due`] and [`Todo::completed`] now take their date-time argument by value rather than by
//!   reference
//! - [`Todo::completed`] now requires its [`chrono::DateTime`] argument to have exactly [`chrono::Utc`]
//!   specified as its time zone as mandated by the RFC.
//! - [`EventLike::starts`], [`EventLike::ends`] and [`Todo::due`] now take newly introduced
//!   [`CalendarDateTime`] (through [`Into<CalendarDateTime>`] indirection). This allows callers to
//!   define time zone handling. Conversions from [`chrono::NaiveDateTime`] and
//!   [`chrono::DateTime<Utc>`](chrono::DateTime) are provided for ergonomics, the latter also restoring API
//!   compatibility in case of UTC date-times.

#![allow(deprecated)]
#![warn(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_debug_implementations,
    clippy::indexing_slicing,
    clippy::dbg_macro,
    clippy::doc_markdown,
    clippy::redundant_closure_for_method_calls
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

#[cfg(all(test, feature = "parser"))]
#[macro_use]
mod assert;

//pub mod period;
mod calendar;
mod components;
#[cfg(feature = "parser")]
pub mod parser;
mod properties;

pub use crate::{
    calendar::{Calendar, CalendarComponent},
    components::{
        alarm::{Alarm, Related, Trigger},
        date_time::{CalendarDateTime, DatePerhapsTime},
        Component, Event, EventLike, Todo, Venue,
    },
    properties::{Class, EventStatus, Parameter, Property, TodoStatus, ValueType},
};

#[cfg(feature = "chrono-tz")]
pub use crate::components::date_time::ymd_hm_tzid;

// TODO Calendar TimeZone VTIMEZONE STANDARD DAYLIGHT (see thunderbird exports)
