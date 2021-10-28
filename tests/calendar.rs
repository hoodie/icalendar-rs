use chrono::prelude::*;
use icalendar::{Calendar, Class, Component, Event, EventStatus, Todo};
use pretty_assertions::assert_eq;

const EXPECTED_CAL_CONTENT: &str = "\
BEGIN:VCALENDAR\r
VERSION:2.0\r
PRODID:ICALENDAR-RS\r
CALSCALE:GREGORIAN\r
BEGIN:VEVENT\r
CLASS:CONFIDENTIAL\r
DESCRIPTION:Description\r
DTEND:20140709T091011Z\r
DTSTAMP:20190307T181159\r
DTSTART:20140708T071011Z\r
LOCATION:Somewhere\r
PRIORITY:10\r
STATUS:TENTATIVE\r
SUMMARY:summary\r
TEST:A\r
TEST:C\r
TEST:D\r
UID:euid\r
END:VEVENT\r
BEGIN:VTODO\r
COMPLETED:20140709T091011Z\r
DTSTAMP:20190307T181159\r
DUE:20140708T091011\r
PERCENT-COMPLETE:95\r
SUMMARY:A Todo\r
TEST:B\r
TEST:C\r
UID:todouid\r
END:VTODO\r
END:VCALENDAR\r
";

#[test]
fn test_calendar_to_string() {
    let mut calendar = Calendar::new();
    let cest_date = FixedOffset::east(2 * 3600)
        .ymd(2014, 7, 8)
        .and_hms(9, 10, 11);
    let utc_date = Utc.ymd(2014, 7, 9).and_hms(9, 10, 11);
    let event = Event::new()
        .status(EventStatus::Tentative)
        .starts(cest_date.with_timezone(&Utc))
        .ends(utc_date)
        .priority(11) // converted to 10
        .summary("summary")
        .description("Description")
        .location("Somewhere")
        .uid("euid")
        .class(Class::Confidential)
        .property("DTSTAMP", "20190307T181159")
        .multi_property("TEST", ["A", "B"])
        .multi_property("TEST", ["A", "C"])
        .multi_property_appended("TEST", ["D"]);
    calendar.push(event);
    let todo = Todo::new()
        .percent_complete(95)
        .due(cest_date.naive_local())
        .completed(utc_date)
        .summary("A Todo")
        .uid("todouid")
        .property("DTSTAMP", "20190307T181159")
        .property("TEST", "A")
        .property("TEST", "B")
        .property_appended("TEST", "C");
    calendar.push(todo);
    assert_eq!(calendar.to_string(), EXPECTED_CAL_CONTENT);
}
