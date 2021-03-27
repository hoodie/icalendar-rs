#![allow(missing_docs)]
use nom::{error::convert_error, error::VerboseError, Finish, IResult};

mod components;
mod parameters;
mod properties;
mod utils;

use components::*;
use properties::*;
use utils::normalize;

fn read_calendar(input: &str) -> IResult<&str, Vec<Component<'_>>, VerboseError<&str>> {
    components::components(input)
}

pub fn calendar(sample: &str) {
    let normalized = normalize(&sample);
    println!(
        "{}",
        normalized
            .lines()
            .enumerate()
            .map(|(num, content)| format!("{}. {}\n", num + 1, content))
            .collect::<String>()
    );
    match read_calendar(&normalized).finish() {
        Ok((_, read)) => println!("{:#?}", read),
        Err(e) => println!("error: {}", convert_error(normalized.as_str(), e.clone())),
    };
}
