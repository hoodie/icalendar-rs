#![allow(missing_docs)]
// #![allow(dead_code, unused_variables, unused_imports)]

mod utils;
//mod lines;

////////// Parameters
mod parameters;

////////// Properties
pub mod properties;
use nom::{error::convert_error, error::VerboseError, Err, IResult};
use properties::*;

////////// Components
pub mod components;
use components::*;

pub use utils::{simplify_line_endings, unfold};

fn read_calendar(input: &str) -> IResult<&str, Vec<Component<'_>>, VerboseError<&str>> {
    components::components(input)
}

pub fn calendar(sample: &str) {
    let normalized = simplify_line_endings(&sample);
    let unfolded = unfold(&normalized);
    println!(
        "{}",
        unfolded
            .lines()
            .enumerate()
            .map(|(num, content)| format!("{}. {}\n", num+1, content))
            .collect::<String>()
    );
    match read_calendar(&unfolded) {
        Ok((_, read)) => {
            println!("{:#?}", read)
        }
        Err(Err::Failure(e)) => {
            println!("error: {}", convert_error(unfolded.as_str(), e.clone()))
        }
        Err(Err::Error(e)) => {
            println!("error: {}", convert_error(unfolded.as_str(), e.clone()))
        }
        Err(Err::Incomplete(e)) => println!("error: {:?}", e),
    };
}
