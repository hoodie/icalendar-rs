#![allow(missing_docs)]
use nom::{error::convert_error, error::VerboseError, Finish, IResult};

pub(crate) mod components;
mod parameters;
mod properties;
mod utils;
#[cfg(test)]
mod tests;

use components::*;
use properties::*;
use utils::normalize;

fn read_calendar(input: &str) -> IResult<&str, Vec<Component<'_>>> {
    components(input)
}

pub fn read_calendar_verbose(input: &str) -> Result<Vec<Component<'_>>, String> {
    components(input)
        .finish()
        .map(|(_, components)| components)
        .map_err(|e: VerboseError<&str>| format!("error: {}", convert_error(input, e.clone())))
}

pub fn calendar(sample: &str, verbose: bool) {
    let normalized = normalize(sample);
    println!(
        "{}",
        normalized
            .lines()
            .enumerate()
            .map(|(num, content)| format!("{:4}. {}\n", num + 1, content))
            .collect::<String>()
    );
    if verbose {
        match read_calendar_verbose(&normalized) {
            Ok(read) => println!("{:#?}", read),
            Err(e) => println!("error: {}", e),
        };
    } else {
        match read_calendar(&normalized).finish() {
            Ok((_, read)) => println!("{:#?}", read),
            Err(e) => println!("error: {}", e),
        };
    }
}
