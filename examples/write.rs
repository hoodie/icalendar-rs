extern crate chrono;
extern crate icalendar;
use chrono::*;
use icalendar::*;

fn main() {
    let event = Event::new()
        .summary("test event")
        .description("here I have something really important to do")
        .starts(UTC::now())
        .class(Class::Confidential)
        //.repeats_every(15.days())
        //.repeats(Every::second().wednesday())
        .ends(UTC::now() + Duration::days(1))
        //.all_day()
        .append_property(Property::new("TEST", "FOOBAR")
                  .add_parameter("IMPORTANCE", "very")
                  .add_parameter("DUE", "tomorrow")
                  .done())
        .done();

    let bday = Event::new()
        .start_date(UTC.ymd(2016, 3, 15))
        .end_date(UTC.ymd(2016, 3, 15))
        .summary("My Birthday")
        .description(
r#"Hey, I'm gonna have a party
BYOB: Bring your own beer.
Hendrik"#
)
        .done();

    let bday2 = Event::new()
        .all_day(UTC.ymd(2016, 3, 15))
        .done();

    let todo = Todo::new().summary("Buy some milk").done();


    let mut calendar = Calendar::new();
    calendar.add(event);
    calendar.add(todo);
    calendar.add(bday);
    calendar.add(bday2);

    calendar.print().unwrap();
}
