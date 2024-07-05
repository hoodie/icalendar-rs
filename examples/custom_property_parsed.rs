#![cfg(feature = "parser")]
use icalendar::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event = Event::new()
        .summary("test event")
        .append_property(
            r#"TEST;IMPORTANCE=very;DUE=tomorrow:FOOBAR;COMPLEX=\n"#
                .parse::<Property>()
                .unwrap(),
        )
        // .uid("my.own.id")
        .done();

    let parsed_event = r#"BEGIN:VEVENT
DTSTAMP:20211123T192118Z
SUMMARY:parsed event
TEST;IMPORTANCE=very;DUE=tomorrow:FOOBAR
END:VEVENT
"#;

    let calendar = Calendar::from(event)
        .push(parsed_event.parse::<CalendarComponent>().unwrap())
        .done();
    calendar.print()?;

    let parsed_calendar = calendar.to_string().parse::<Calendar>()?;
    parsed_calendar.print()?;
    Ok(())
}
