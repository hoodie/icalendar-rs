use chrono::*;
use icalendar::*;

fn main() {
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
        .all_day(Utc.ymd(2016, 3, 15))
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
