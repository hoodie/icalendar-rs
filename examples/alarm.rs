use chrono::*;
use icalendar::*;

fn main() {
    let mut calendar = Calendar::new();

    let now = Utc::now();
    let soon = Utc::now() + Duration::minutes(12);
    let tomorrow = Utc::now() + Duration::days(1);

    let audio = Alarm::audio(-Duration::minutes(10))
        .duration_and_repeat(chrono::Duration::minutes(1), 4)
        .uid("todo_test_audio_alarm")
        .done();

    let display = Alarm::display(
        "you should test your implementation",
        Utc::now() + Duration::minutes(1),
    )
    .duration_and_repeat(chrono::Duration::minutes(1), 4)
    .uid("todo_test_display_alarm")
    .done();

    let display2 = Alarm::display(
        "you should test your implementation",
        -Duration::minutes(10),
    )
    .duration_and_repeat(chrono::Duration::minutes(1), 4)
    .uid("todo_test_display_alarm")
    .done();

    let audio_abs = Alarm::audio(now + Duration::minutes(1))
        .duration_and_repeat(chrono::Duration::minutes(1), 4)
        .uid("todo_taxes_alarm")
        .done();
}
