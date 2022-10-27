use chrono::*;
use icalendar::*;

fn main() {
    // lets create a calendar
    let my_calendar = Calendar::new()
        .name("example calendar")
        .push(
            Event::new()
                .summary("test event")
                .description("here I have something really important to do")
                .starts(Utc::now())
                .class(Class::Confidential)
                .ends(Utc::now() + Duration::days(1))
                .alarm(Duration::days(1), Action::Audio)
                .append_component(
                    Alarm::with_trigger(Trigger::from(Duration::days(1))).and_action(Action::Audio),
                )
                .done(),
        )
        .done();

    println!("{}", my_calendar);
}
