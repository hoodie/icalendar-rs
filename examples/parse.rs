use std::{env::args, fs::read_to_string};

use icalendar::parse::calendar;

fn main() {
    if let Some(sample) = args().nth(1).map(read_to_string) {
        let verbose = true;
        calendar(&sample.unwrap(), verbose);
    }
}
