use chrono::*;
use icalendar::*;

fn main() {
    // lets create a calendar
    let my_calendar = Calendar::new()
        .name("example calendar")
        .push(
            Event::new()
                // .summary("test event")
                // .description("here I have something really important to do")
                // .starts(Utc::now())
                // .class(Class::Confidential)
                // .ends(Utc::now() + Duration::days(1))

                // .alarm((Duration::days(1), Related::End), Action::Audio)
                // .alarm(Utc::now(), Action::Audio)
                .alarm(Alarm::audio((Duration::minutes(15), Related::Start)))
                .done(),
        )
        .done();

    println!("{}", my_calendar);
}
