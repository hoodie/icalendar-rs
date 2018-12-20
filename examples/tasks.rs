
extern crate icalendar;
extern crate chrono;
use icalendar::*;
use chrono::*;

fn main(){

    let todo = Todo::new()
        .starts(Local::now())
        .ends(Local::now())
        .priority(12)
        .percent_complete(28)
        .status(TodoStatus::Completed)
        .completed(&Local::now())
        .due(&Local::now())
        .due(&Local::now())
        .done();

    println!("{}", todo.to_string());
}
