use chrono::*;
use icalendar::*;

fn main() {
    let todo = Todo::new()
        .starts(Local::now().naive_local())
        .ends(Local::now().naive_local())
        .priority(12)
        .percent_complete(28)
        .status(TodoStatus::Completed)
        .completed(Utc::now())
        .due(Local::now().with_timezone(&Utc));

    println!("{}", todo.to_string());
}
