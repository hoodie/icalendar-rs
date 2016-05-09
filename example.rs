
extern crate icalendar;
use icalendar::Event;
use icalendar::every;

fn main(){
    let event = Event::new()
        .start_date(chrono::Date());
        .end_date(chrono::Date());
        .summary(Some(String))
        .description(Some(String))
        .attendee("mailto:john.doe@example.com")
        .attendee(Attendee::new("jane.doe@hoodie.de").has_declined())
        .attendee(Attendee::new("jane.doe@hoodie.de").has_accepted())
        .trigger("-PT15M")
        .repeats_every(15.days())
        .repeats_every(Repeats::First.wednesday())
        .repeats_every(Repeats::Last.friday().in(Month::June))
        .busy(between("09.05.2016").an("12.05.2016") )
        .ip_class(IpClass::Private)
        ;
}

// BEGIN:VCALENDAR
// VERSION:2.0
// PRODID:icalendar-rust
// CALSCALE:GREGORIAN
//
// BEGIN:VEVENT
// DTSTAMP:20160507T120700Z
// UID:6e184fba-6e9a-484f-8b47-384f771276c4
// DTSTART;VALUE=DATE:20121210
// DTEND;VALUE=DATE:20121211
// DESCRIPTION:Verantwortung: Frank Hedecke\n
// SUMMARY:Weihnachtsfeier
// END:VEVENT
//
// BEGIN:VEVENT
// DTSTAMP:20160507T120700Z
// UID:bc8b15ec-fa3d-4b1d-811f-4a7277755122
// DTSTART;VALUE=DATE:20130120
// DTEND;VALUE=DATE:20130121
// DESCRIPTION:Verantwortung: Hendrik Sollich\n
// END:VEVENT
