<div align="center">

# iCalendar in Rust

[![build](https://img.shields.io/github/actions/workflow/status/hoodie/icalendar-rs/ci.yml?branch=main)](https://github.com/hoodie/icalendar-rs/actions?query=workflow%3A"Continuous+Integration")
[![Crates.io](https://img.shields.io/crates/d/icalendar)](https://crates.io/crates/icalendar)
[![contributors](https://img.shields.io/github/contributors/hoodie/icalendar-rs)](https://github.com/hoodie/icalendar-rs/graphs/contributors)
![maintenance](https://img.shields.io/maintenance/yes/2025)

[![version](https://img.shields.io/crates/v/icalendar)](https://crates.io/crates/icalendar/)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/icalendar/)
[![license](https://img.shields.io/crates/l/icalendar.svg?style=flat)](https://crates.io/crates/icalendar/)

A builder [and parser] for [`rfc5545`](http://tools.ietf.org/html/rfc5545) iCalendar.
</div>


You want to help make this more mature? Please talk to me, Pull Requests and suggestions are very welcome.

## Example

Use the builder-pattern to assemble the full calender or event by event.
Display printing produces the rfc5545 format.

```rust
// lets create a calendar
let my_calendar = Calendar::new()
    .name("example calendar")
    .push(
        // add an event
        Event::new()
            .summary("test event")
            .description("here I have something really important to do")
            .starts(Utc::now())
            .class(Class::Confidential)
            .ends(Utc::now() + Duration::days(1))
            .append_property(
                Property::new("TEST", "FOOBAR")
                    .add_parameter("IMPORTANCE", "very")
                    .add_parameter("DUE", "tomorrow")
                    .done(),
            )
            .done(),
    )
    .push(
        // add a todo
        Todo::new()
            .summary("groceries")
            .description("Buy some milk")
            .done(),
    )
    .push(
        // add an all-day event
        Event::new()
            .all_day(NaiveDate::from_ymd_opt(2016, 3, 15).unwrap())
            .summary("My Birthday")
            .description("Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
            .done(),
    )
    .push(
        // local event with timezone
        Event::new()
            .starts(CalendarDateTime::from_ymd_hm_tzid(2023, 3, 15, 18, 45, Berlin).unwrap())
            .summary("Birthday Party")
            .description("I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
            .done(),
    )
    .done();

println!("{}", my_calendar);

```

## Parsing
There is a feature called `"parser"` which allows you to read calendars again like this:

```rust
//... continue from previous example

let parsed_calendar = my_calendar.parse::<Calendar>()?;
```

## License

icalendar-rs is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any help in form of descriptive and friendly [issues](https://github.com/hoodie/icalendar-rs/issues) or comprehensive pull requests are welcome! 

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in icalendar-rs by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
