use chrono::*;
use icalendar::*;

fn main() {
    let event = Event::new()
        .summary("test event")
        .description("here I have something really important to do")
        .starts(Utc::now())
        .class(Class::Confidential)
        .ends(Utc::now() + Duration::days(1))
        .property(
            "TEST",
            Property::from("FOOBAR")
                .parameter("IMPORTANCE", "very")
                .parameter("DUE", "tomorrow"),
        );

    let bday = Event::new()
        .all_day(Utc.ymd(2020, 3, 15))
        .summary("My Birthday")
        .description(
            r#"Hey, I'm gonna have a party
    BYOB: Bring your own beer.
    Hendrik"#,
        );

    let todo = Todo::new().summary("Buy some milk");

    let mut calendar = Calendar::new();
    calendar.push(event);
    calendar.push(todo);
    calendar.push(bday);
}
