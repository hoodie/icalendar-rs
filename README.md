<div align="center">

# iCalendar in Rust

[![build](https://img.shields.io/github/workflow/status/hoodie/icalendar-rs/Continuous%20Integration)](https://github.com/hoodie/icalendar-rs/actions?query=workflow%3A"Continuous+Integration")
[![Crates.io](https://img.shields.io/crates/d/icalendar)](https://crates.io/crates/icalendar)
[![contributors](https://img.shields.io/github/contributors/hoodie/icalendar-rs)](https://github.com/hoodie/icalendar-rs/graphs/contributors)
![maintenance](https://img.shields.io/maintenance/yes/2021)

[![version](https://img.shields.io/crates/v/icalendar)](https://crates.io/crates/icalendar/)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/icalendar/)
[![license](https://img.shields.io/crates/l/icalendar.svg?style=flat)](https://crates.io/crates/icalendar/)

</div>

A very simple library to generate [`rfc5545`](http://tools.ietf.org/html/rfc5545) calendars.
Please double check the [spec](http://tools.ietf.org/html/rfc5545).

You want to help make this more mature? Please talk to me, Pull Requests and suggestions are very welcome.

## Examples

```rust
let event = Event::new()
    .summary("test event")
    .description("here I have something really important to do")
    .starts(Utc::now())
    .class(Class::Confidential)
    .ends(Utc::now() + Duration::days(1))
    .append_property(Property::new("TEST", "FOOBAR")
            .add_parameter("IMPORTANCE", "very")
            .add_parameter("DUE", "tomorrow")
            .done())
    .done();

let bday = Event::new()
    .all_day(Utc.ymd(2020, 3, 15))
    .summary("My Birthday")
    .description(
r#"Hey, I'm gonna have a party
BYOB: Bring your own beer.
Hendrik"#
)
    .done();

let todo = Todo::new().summary("Buy some milk").done();


let mut calendar = Calendar::new();
calendar.push(event);
calendar.push(todo);
calendar.push(bday);
```

## License

icalendar-rs is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any help in form of descriptive and friendly [issues](https://github.com/hoodie/icalendar-rs/issues) or comprehensive pull requests are welcome! 

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in icalendar-rs by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
