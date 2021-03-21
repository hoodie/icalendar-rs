#![allow(missing_docs)]
// #![allow(dead_code, unused_variables, unused_imports)]

mod utils;
//mod lines;

////////// Parameters
mod parameters;

////////// Properties
pub mod properties;
use properties::*;

////////// Components
pub mod components;
use components::*;

pub use utils::{simplify_line_endings, unfold};

pub fn read_calendar(input: &str) -> Option<Vec<Component<'_>>> {
    components::components(input)
        .map(|(_rest, components)| components)
        .ok()
}
