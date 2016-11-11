//! A library (far from anything) to generate icalendars
//! This is still just an early idea, there is nothing implemented,
//! I haven't even read the [spec](http://tools.ietf.org/html/rfc5545) yet.
//!
//! ## Structure
//! * `Calendar`s consist of `Components`
//! * `Component`s are e.g. `Event` or `Todo`
//! * `Component`s consist of `Property`s
//! * `Property`s may have `Parameter`s

#![allow(unused_variables)]

extern crate chrono;
extern crate uuid;
extern crate vobject;

//pub mod period;
mod components;
mod properties;
mod calendar;

pub mod repeats;
pub use properties::{Property, Parameter, Class, ValueType};
//pub use components::{event, todo};
pub use components::{Event, Todo, Component};
pub use calendar::Calendar;

#[test]
fn it_works() {
    let birthday = Event::new("My Birthday").done();
    //let birthday = Event::new("My Birthday").repeats(Annually).done();
    //let birthday = Event::new("My Birthday").every(15.days()).done();
    println!("{:#?}", birthday);
}

//BEGIN:VEVENT
//DTSTAMP:20161028T133924Z
//UID:ba95bf93-f816-434c-8759-f05a2345b575
//DTSTART:20161021T170000
//DTEND:20161021T140000
//DESCRIPTION: this\n is\n the description.
//SUMMARY:Hackathon Planworx
//END:VEVENT

