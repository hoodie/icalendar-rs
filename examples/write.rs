extern crate chrono;
extern crate icalendar;
extern crate vobject;

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
        .append_property(Property::new("test", "foobar")
                  .add_parameter("IMPORTANCE", "very")
                  .add_parameter("DUE", "tomorrow")
                  .done())
        .done();

    let bday = Event::new()
        .start_date(UTC.ymd(2016, 3, 15))
        .end_date(UTC.ymd(2016, 3, 15))
        .summary("My Birthday")
        .description(
r#"Hier mach ich ja sowas von Part.
Ich könnt gerne alle kommen.

BYOB: Bring your own beer.
Hendrik"#
)
        .done();
    let todo = Todo::new().summary("Buy some milk").done();


    vobject::parse_component(&bday.to_string()).unwrap();
    vobject::parse_component(&todo.to_string()).unwrap();
    vobject::parse_component(&event.to_string()).unwrap();

    let mut calendar = Calendar::new();
    calendar.add(event);
    calendar.add(todo);
    calendar.add(bday);

    calendar.print().unwrap();
}
