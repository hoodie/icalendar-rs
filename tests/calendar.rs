use chrono::*;
use icalendar::*;
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
UID:euid\r
END:VEVENT\r
BEGIN:VTODO\r
COMPLETED:20140709T091011Z\r
DTSTAMP:20190307T181159\r
DUE:20140708T091011\r
PERCENT-COMPLETE:95\r
SUMMARY:A Todo\r
UID:todouid\r
END:VTODO\r
END:VCALENDAR\r
";

#[test]
fn test_calendar_to_string() {
    let mut calendar = Calendar::new();
    let cest_date = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2014, 7, 8, 9, 10, 11)
        .unwrap();
    let utc_date = Utc.with_ymd_and_hms(2014, 7, 9, 9, 10, 11).unwrap();
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
        .add_property("DTSTAMP", "20190307T181159")
        .done();
    calendar.push(event);
    let todo = Todo::new()
        .percent_complete(95)
        .due(cest_date.naive_local())
        .completed(utc_date)
        .summary("A Todo")
        .uid("todouid")
        .add_property("DTSTAMP", "20190307T181159")
        .done();
    calendar.push(todo);
    assert_eq!(calendar.to_string(), EXPECTED_CAL_CONTENT);
}

#[test]
fn test_build_calendar() {
    use chrono::*;
    use icalendar::*;
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
        .done();

    let bday = Event::new()
        .all_day(NaiveDate::from_ymd_opt(2023, 3, 15).unwrap())
        .summary("My Birthday")
        .description(
            r#"Hey, I'm gonna have a party
BYOB: Bring your own beer.
Hendrik"#,
        )
        .done();

    let todo = Todo::new().summary("Buy some milk").done();

    let mut calendar = Calendar::new();
    calendar.push(event);
    calendar.push(todo);
    calendar.push(bday);
}
