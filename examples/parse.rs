use std::{env::args, fs::read_to_string};

use icalendar::parse::{read_calendar, unfold};

fn print_with_lines(content: &str) {
    println!(
        "{}",
        content
            .lines()
            .enumerate()
            .map(|(num, content)| format!("{:4}. {}\n", num + 1, content))
            .collect::<String>()
    );
}

fn main() {
    if let Some(sample) = args().nth(1).map(read_to_string) {
        let normalized = unfold(&sample.unwrap());
        print_with_lines(&normalized);

        match read_calendar(&normalized) {
            Ok(read) => println!("{:#?}", read),
            Err(error) => println!("non-verbose error\n {}", error),
        }

        // match icalendar::parse::read_calendar_simple(&normalized) {
        //     Ok(read) => println!("{:#?}", read),
        //     Err(error) => println!("verbose error\n {}", error),
        // }
    }
}
