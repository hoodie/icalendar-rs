use chrono::prelude::*;
use icalendar::{Calendar, Class, Component, Event, EventStatus};

const EXPECTED_CAL_CONTENT: &str = "\
BEGIN:VCALENDAR\r
VERSION:2.0\r
PRODID:ICALENDAR-RS\r
CALSCALE:GREGORIAN\r
BEGIN:VEVENT\r
CLASS:CONFIDENTIAL\r
DESCRIPTION:Description\r
DTEND:20140709T091011\r
DTSTAMP:20190307T181159\r
DTSTART:20140708T091011\r
LOCATION:Somewhere\r
PRIORITY:10\r
STATUS:TENTATIVE\r
SUMMARY:summary\r
UID:euid\r
END:VEVENT\r
END:VCALENDAR\r
";

#[test]
fn test_calendar_to_string() {
    let mut calendar = Calendar::new();
    let event = Event::new()
        .status(EventStatus::Tentative)
        .starts(Local.ymd(2014, 7, 8).and_hms(9, 10, 11))
        .ends(Local.ymd(2014, 7, 9).and_hms(9, 10, 11))
        .priority(11) // converted to 10
        .summary("summary")
        .description("Description")
        .location("Somewhere")
        .uid("euid")
        .class(Class::Confidential)
        .add_property("DTSTAMP", "20190307T181159")
        .done();
    calendar.push(event);
    assert_eq!(calendar.to_string(), EXPECTED_CAL_CONTENT);
}
