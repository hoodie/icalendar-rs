#![cfg(feature = "parser")]
use std::{env, fs, io};

use icalendar::parser::unfold;

mod example_utils;
use example_utils::{content_from_arg, print_with_lines};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(sample) = content_from_arg()? {
        let unfolded = unfold(&sample);
        print_with_lines(&unfolded);

        let human_readable = !true;

        if human_readable {
            match icalendar::parser::read_calendar(&unfolded) {
                Ok(read) => println!("{:#?}", read),
                Err(error) => println!("human-readable error\n{}", error),
            }
        } else {
            match icalendar::parser::read_calendar_simple(&unfolded) {
                Ok(read) => println!("{:#?}", read),
                Err(error) => println!("plain error\n{:#?}", error),
            }
        }
    }
    Ok(())
}
