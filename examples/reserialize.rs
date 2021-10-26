use std::{convert::TryFrom, env::args, fs::read_to_string};

use icalendar::{
    parse::{normalize, read_calendar},
    Calendar,
};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(sample) = args().nth(1).map(read_to_string) {
        let normalized = normalize(&sample.unwrap());
        print_with_lines(&normalized);

        let components = read_calendar(&normalized)?;

        let calendar = Calendar::try_from(components).unwrap();
        
        print_with_lines(&calendar.to_string());
    }
    Ok(())
}
