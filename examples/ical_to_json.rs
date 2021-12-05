#![cfg(all(feature = "parser", feature = "serde"))]
use icalendar::*;

mod example_utils;
use example_utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(sample) = content_from_arg()? {
        let parsed_calendar = match sample.parse::<Calendar>() {
            Ok(read) => read,
            Err(error) => {
                println!("{}", error); // println!(error) yields prettier output
                return Ok(());
            }
        };
        println!("{}", serde_json::to_string_pretty(&parsed_calendar)?);
    }
    Ok(())
}
