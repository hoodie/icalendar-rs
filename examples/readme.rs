use chrono::*;
use icalendar::*;

fn main() {
    let my_calendar = Calendar::new()
        .push(
            Event::new()
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
                .done(),
        )
        .push(
            Todo::new()
                .summary("groceries")
                .description("Buy some milk")
                .done(),
        )
        .push(
            Event::new()
                .all_day(Utc.ymd(2016, 3, 15))
                .summary("My Birthday")
                .description("Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
                .done(),
        )
        .done();

    println!("{}", my_calendar)
}
