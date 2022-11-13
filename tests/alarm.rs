use chrono::*;
use icalendar::*;
use pretty_assertions::assert_eq;

/// Taken from https://datatracker.ietf.org/doc/html/rfc5545
/// some properties have been removed or reordered
const EXPECTED_CAL_CONTENT: &str = "\
BEGIN:VCALENDAR\r
VERSION:2.0\r
PRODID:ICALENDAR-RS\r
CALSCALE:GREGORIAN\r
BEGIN:VTODO\r
DTSTAMP:19980130T134500Z\r
DTSTART:19980130T134500Z\r
DUE:19980415T000000\r
SEQUENCE:2\r
STATUS:NEEDS-ACTION\r
SUMMARY:Submit Income Taxes\r
UID:uid4@example.com\r
BEGIN:VALARM\r
ACTION:AUDIO\r
DTSTAMP:19980130T134500Z\r
DURATION:PT3600S\r
REPEAT:4\r
TRIGGER:19980403T120000Z\r
UID:OverwriteForConsistency\r
END:VALARM\r
END:VTODO\r
END:VCALENDAR\r
";

#[test]
fn test_alarm_to_string() {
    let mut calendar = Calendar::new();
    let todo = Todo::new()
        .uid("uid4@example.com")
        .add_property("DTSTAMP", "19980130T134500Z")
        .sequence(2)
        //.organizer("")
        .starts(Utc.ymd(1998, 1, 30).and_hms(13, 45, 0))
        // .due(Utc.ymd(1998, 4, 15).and_hms(0, 0, 0))
        .due(NaiveDate::from_ymd(1998, 4, 15).and_hms(0, 0, 0))
        //.repeat(4)
        .status(TodoStatus::NeedsAction)
        .summary("Submit Income Taxes")
        // .append_component(
        //     Alarm::with_trigger(Trigger::from(Utc.ymd(1998, 4, 3).and_hms(12, 0, 0)))
        //         .duration(chrono::Duration::hours(1))
        //         .uid("OverwriteForConsistency")
        //         .action(Action::Audio)
        //         .repeat(4)
        //         .add_property("DTSTAMP", "19980130T134500Z")
        //         .done(),
        // )
        .done();
        todo!();
    calendar.push(todo);
    assert_eq!(calendar.to_string(), EXPECTED_CAL_CONTENT);
}
