use std::{env::args, fs::read_to_string};

use icalendar::parse::{read_calendar, simplify_line_endings, unfold};

fn main() {
    if let Some(sample) = args().nth(1).map(read_to_string) {
        let normalized = simplify_line_endings(&unfold(&sample.unwrap()));
        dbg!(read_calendar(&normalized));
    }
}
