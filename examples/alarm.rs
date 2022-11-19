use chrono::*;
use icalendar::*;

fn main() {
    let mut calendar = Calendar::new();
    let todo = Todo::new()
        .uid("uid4@example.com")
        .add_property("DTSTAMP", "19980130T134500Z")
        .sequence(2)
        //.organizer("")
        .starts(Utc.with_ymd_and_hms(1998, 1, 30, 13, 45, 0).unwrap())
        // .due(Utc.ymd(1998, 4, 15).and_hms(0, 0, 0))
        .due(
            NaiveDate::from_ymd_opt(1998, 4, 15)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .status(TodoStatus::NeedsAction)
        .summary("Submit Income Taxes")
        .append_component(
            Alarm::audio(
                Utc.ymd_opt(1998, 4, 3)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
            )
            .duration_and_repeat(chrono::Duration::hours(1), 4)
            .uid("OverwriteForConsistency")
            .add_property("DTSTAMP", "19980130T134500Z")
            .done(),
        )
        .done();
    calendar.push(todo);

    #[cfg(feature = "parser")]
    {
        use std::str::FromStr;

        let source = calendar.to_string();
        //let reparse = icalendar::parser::read_calendar(&source);
        let reparse = Calendar::from_str(&source).unwrap();
        println!("{:#?}", reparse);
    }
}
