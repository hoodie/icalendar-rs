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
//! # extern crate chrono;
//! # extern crate icalendar;
//! # use chrono::*;
//! # use icalendar::*;
//! let event = Event::new()
//!     .summary("test event")
//!     .description("here I have something really important to do")
//!     .starts(UTC::now())
//!     .class(Class::Confidential)
//!     .ends(UTC::now() + Duration::days(1))
//!     .append_property(Property::new("TEST", "FOOBAR")
//!               .add_parameter("IMPORTANCE", "very")
//!               .add_parameter("DUE", "tomorrow")
//!               .done())
//!     .done();
//!
//! let bday = Event::new()
//!     .all_day(UTC.ymd(2016, 3, 15))
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
//! calendar.add(event);
//! calendar.add(todo);
//! calendar.add(bday);
//!
//! ```

#![warn(missing_docs,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications,
        missing_debug_implementations
        )]

extern crate chrono;
extern crate uuid;
//extern crate vobject;

//pub mod period;
mod components;
mod properties;
mod calendar;

//pub mod repeats;
pub use properties::{Property, Parameter, Class, ValueType};
pub use properties::{TodoStatus, EventStatus};
//pub use components::{event, todo};
pub use components::{Event, Todo, Component};
pub use calendar::Calendar;

// TODO Calendar TimeZone VTIMEZONE STANDARD DAYLIGHT (see thunderbird exports)

