#![cfg(feature = "parser")]
use icalendar::parser::unfold;

mod example_utils;
use example_utils::{content_from_arg, print_with_lines};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(content) = content_from_arg()? {
        // print_with_lines(&unfolded);

        let calendar = serde_json::from_str::<icalendar::parser::Calendar>(&content)?;
        println!("{}", calendar);
    }
    Ok(())
}
