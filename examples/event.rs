use chrono::*;
use icalendar::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event = Event::new()
        .summary("test event")
        .description("here I have something really important to do")
        .starts(Utc::now())
        .class(Class::Confidential)
        .ends(Utc::now() + Duration::days(1))
        .done();

    let event2 = Event::new().all_day(Utc.ymd(2016, 3, 15)).done();

    let calendar = Calendar::from([event, event2]);

    calendar.print()?;

    #[cfg(feature = "parser")]
    {
        let parsed_calendar = dbg!(calendar.to_string().parse::<Calendar>()?);
        parsed_calendar.print()?;
    }
    Ok(())
}
