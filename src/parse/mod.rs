//! # Parsing iCalendar document parser
//!
//! I would have loved to provide a zero-copy parser here however the *Internet Calendaring and Scheduling Core Object Specification (iCalendar)*
//! allows, nay demands special line folding
//! >    Lines of text SHOULD NOT be longer than 75 octets, excluding the line break.  Long content lines SHOULD be split into a multiple line representations using a line "folding" technique.
//! > -- [rfc5545 3.1]
//!
//! For this reason parsing iCal is a bit indirect.
//! In this module you find the following functions to parse iCalendar document.
//! `normalize()` will unfold the iCal content and turn it into the nice machine-readable format it ought to be.
//! `read_calendar()` returns a Vector of `Component`s
//! `read_calendar_verbose()` does the same thing but produces nicer parsing errors with line numbers (referencing the normalized content).
//!
//! You don't have to use `normalize()` on your document if your calandar does not obey the folding rules specified in [rfc5545 3.1].
//! If it unexpectedly does, the errors might be a tad confusing.
//!
//!
//! [rfc5545 3.1]: https://datatracker.ietf.org/doc/html/rfc5545#section-3.1
//! 
//! A Calendar is always a [`Vec`] of [`Component`]. Each [`Component`] has a [`Vec`] of [`Property`] and those have of [`Parameter`]s.
//!
//! 
#![allow(missing_docs)]
use nom::{error::convert_error, error::VerboseError, Finish};

pub(crate) mod components;
mod parameters;
mod properties;
#[cfg(test)]
mod tests;
mod utils;

pub use components::Component;
pub use parameters::Parameter;
pub use properties::Property;

use components::*;

pub use utils::normalize;

/// Parse iCalendar file content into an array of [`Component`]s
/// 
/// This version produces very simple Errors for simplicity's sake.
pub fn read_calendar_simple<'a>(input: &'a str) -> Result<Vec<Component<'_>>, nom::error::Error<&'a str>> {
    components(input).finish().map(|(_, components)| components)
}

/// Parse iCalendar file content into an array of [`Component`]s
/// 
/// This version produces nice and readable errors with line numbers thanks the the awesomeness of [`nom`].
/// Line numbers are in regard to the normalized/unfolded version of the input, so better keep those around for reference.
/// 
pub fn read_calendar(input: &str) -> Result<Vec<Component<'_>>, String> {
    components(input)
        .finish()
        .map(|(_, components)| components)
        .map_err(|e: VerboseError<&str>| format!("error: {}", convert_error(input, e.clone())))
}
