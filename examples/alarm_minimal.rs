use chrono::*;
use icalendar::*;

fn main() {
    // alarm will occur one minute from now
    let event_with_absolute_audio_alarm = Event::new()
        .alarm(
            Alarm::audio(Utc::now() + Duration::minutes(1))
                .duration_and_repeat(Duration::minutes(1), 4),
        )
        .done();

    // alarm will occur one minute before the start
    let event_with_relative_display_alarm = Event::new()
        .alarm(
            Alarm::display("ALARM! ALARM!", -Duration::minutes(1))
                .duration_and_repeat(Duration::minutes(1), 4),
        )
        .done();
    // alarm will occur one minute before the end
    let event_with_relative_display_alarm_end = Event::new()
        .alarm(
            Alarm::display("ALARM! ALARM!", (-Duration::minutes(1), Related::End))
                .duration_and_repeat(Duration::minutes(1), 4),
        )
        .done();
    event_with_absolute_audio_alarm.print().unwrap();
    event_with_relative_display_alarm.print().unwrap();
    event_with_relative_display_alarm_end.print().unwrap();
}
