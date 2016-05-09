# iCalendar in Rust

This is still just an early idea, there is nothing implemented,
I haven't even read the completely [spec](http://tools.ietf.org/html/rfc5545) yet.

I'd love to create iCal files with a very [diesel](https://diesel.rs/) or [active support](https://github.com/wycats/rust-activesupport) like syntax.

```rust
fn main(){

    use Repeats::*;
    let important_meeting
        .start_date(chrono::Date::new("2016.05.09"))
        .repeats(BiWeekly);

    let important_meeting
        .start_date(chrono::Date::new("2016.05.09"))
        .every(7.days());

    let birthday = Event::new()
        .start_date(chrono::Date::new("1987.03.15"))
        .repeats(Annually);

    let birthday = Event::new()
        .every(15.march());

    let xmas = Event::every(24.december());


    use Month::*;
    let sysadminday = Event::every(Last.friday().of(July));

    let tdo = Todo::new("buy milk");

    let tdo = Alarm::new("get up").at("05:30").every(Thursday);
    let tdo = Alarm::new("get up").at("06:00").on(Fridays);

    let dont_disturb = Busy::between("09.05.2016").and("12.05.2016");
    let dont_disturb = Busy::between("09..12").am().on(Mondays);

    let dear_diary = Journal::from_description("Dear Diary\nToday my cat ran away. Now I'm sad");

    let brush_teeth = Alarm::new("Brush Your Teeth").twice().every(Day);

    let event = Event::new("this is a summary")
        .start_date(chrono::Date())
        .end_date(chrono::Date())
        .description("This property provides a more complete description of the
                           calendar component than that provided by the \"SUMMARY\" property.")
        .attendee("mailto:john.doe@example.com")
        .attendee(Attendee::new("jane.doe@hoodie.de").has_declined())
        .attendee(Attendee::new("jane.doe@hoodie.de").has_accepted())
        .trigger("-PT15M")
        .repeats_every(Repeats::First.wednesday())
        .ip_class(IpClass::Private)
        ;

}
```

# Steps

1. flesh out an api
2. implement types and datastructures
3. implement serialization
4. find some way to test it



Perhaps you can already use [vobject](http://rust-vobject.unterwaditzer.net/vobject/) to parse iCalendar files.
I need to test this still.


