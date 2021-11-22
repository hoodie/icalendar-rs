use chrono::*;
use icalendar::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let event2 = Event::new().all_day(Utc.ymd(2016, 3, 15)).done();

    let todo = Todo::new().summary("Buy some milk").done();

    let mut calendar = Calendar::from([event, event2]);
    calendar.push(todo);

    calendar.print()?;

    #[cfg(feature = "parser")]
    {
        use std::str::FromStr;
        let parsed_calendar = dbg!(Calendar::from_str(&calendar.to_string())?);
        parsed_calendar.print()?;
    }
    Ok(())
}
