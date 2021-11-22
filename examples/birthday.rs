use chrono::*;
use icalendar::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let birthday = Event::new()
        .start_date(Utc.ymd(2016, 3, 15))
        .end_date(Utc.ymd(2016, 3, 15))
        .summary("My Birthday")
        .description(
            r#"Hey, I'm gonna have a party
BYOB: Bring your own beer.
Hendrik"#,
        )
        .done();

    let calendar = Calendar::from(birthday)
        .name("hendriks birthday calendar")
        .done();
    calendar.print()?;

    #[cfg(feature = "parser")]
    {
        use std::str::FromStr;
        let parsed_calendar = dbg!(Calendar::from_str(&calendar.to_string())?);
        parsed_calendar.print()?;
    }
    Ok(())
}
