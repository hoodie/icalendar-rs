use std::str::FromStr;

use chrono::*;
use icalendar::{
    parse::{read_calendar, unfold},
    Calendar, Class, Component, Event, Property, Todo,
};

fn main() {
    let event = Event::new()
        .summary("test event")
        .description("here I have something really important to do")
        .starts(Utc::now())
        .class(Class::Confidential)
        .ends(Utc::now() + Duration::days(1))
        .append_property(
            Property::new("TEST", "FOOBAR")
                .add_parameter("IMPORTANCE", "very")
                .add_parameter("DUE", "tomorrow")
                .done(),
        )
        .uid("my.own.id")
        .done();

    let todo = Todo::new().summary("Buy some milk").done();

    let mut built_calendar = Calendar::new();
    built_calendar.push(event);
    built_calendar.push(todo);

    // lets write this as **rfc5545**
    let ical = built_calendar.to_string();

    // and now lets parse it again
    let from_parsed = Calendar::from_str(&ical).unwrap();

    println!("{}", &ical); // print what we built
    println!("{}", from_parsed); // print what parsed
    println!("{:#?}", built_calendar); // inner representation of what we built
    println!("{:#?}", from_parsed); // inner representation of what we built and then parsed
    println!("{:#?}", read_calendar(&unfold(&ical)).unwrap()); // inner presentation of the parser's data structure
}
