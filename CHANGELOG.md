# Changelog

## [v0.13.0](https://github.com/hoodie/icalendar-rs/compare/v0.12.1...v0.13.0) (2022-04-20)

### ⚠ BREAKING CHANGE

* Adds a new variant to `CalendarDateTime` enum.
* Changes the type of `Component::all_day`.
* Removes `Component::start_date` and`Component::end_date`.
* Changes the type of `Component::starts` and`Component::ends`.
* Changes the type of `Component::timestamp` and`Component::get_timestamp`.


### Features

* Add support for date-time with timezone.
 e914853

* Add getters for Parameter key and value.
 472f4ca

* Accept Into<DatePerhapsTime> for starts and ends.
 91b1d7d


### Fixes

* Take NaiveDate for all_day.
 193c3cf

* Remove start_date and end_date.
 c0c67fb

* According to RFC5545, DTSTAMP must be in UTC.
 9d2fb43


### [v0.12.1](https://github.com/hoodie/icalendar-rs/compare/v0.12.0...v0.12.1) (2022-04-15)


#### Features

* Add getter and setter for URL property.
 eb53e1a


#### Fixes

* Export DatePerhapsTime type.
 8a46f0c

* Fix typo in README.
 7d117b2


## [v0.12.0](https://github.com/hoodie/icalendar-rs/compare/v0.11.1...v0.12.0) (2022-04-13)


### Features

* Allow due date (without time) to be specified for TODO.
 2a4a451

* Add getters for date-time properties.
 9619316

* Add getter for ValueType on Property.
 293dc70

* Add getters for calendar properties.
 c97acaa

* Add getter for property parameters.
 3702a20

* Add getters for basic todo and event properties.
 dad79f4

* Add getters for venue properties.
 174947a

* Add getters for basic properties.
 437770b

* derive clone and equality traits.
 5d7a8a9


### [v0.11.1](https://github.com/hoodie/icalendar-rs/compare/v0.11.0...v0.11.1) (2022-04-03)


#### Fixes

* build docs for parser feature
 9c2db74


## [v0.11.0](https://github.com/hoodie/icalendar-rs/compare/v0.10.0...v0.11.0) (2022-01-24)


### Features

* add timestamp setter
 c0ffee1

* add serde support
 c0ffee7

* add try_into_string to Component
 c0ffeea

* deprecate fmt_write
 c0ffee0

* add TryInto<String> for &Calendar
 c0ffeed

* add FromStr implementations
 c0ffee2

* expose `CalendarElement`
 684deda

* add missing .done() for `Calendar`
 7bbde0e

* more From implementations for Calendar
 70c70c1

* add FromStr implementation for calendar
 e670469

* reserialize parsed calendar
 64c116d

* simple nom parser
 21c0cfa


### Fixes

* unfold by tabs as well
 c0ffee8

* lost spaces when folding
 c0ffeea

* conversion between parsed Property and Property lost params
 b71826d


## [v0.10.0](https://github.com/hoodie/icalendar-rs/compare/v0.9.0...v0.10.0) (2021-02-12)


### Features

* add propery value method
 c300630


## [v0.9.0](https://github.com/hoodie/icalendar-rs/compare/v0.8.0...v0.9.0) (2020-02-25)


### Features

* Support calendar properties
 1c1ed6b

* Implement iCalendar Venue Draft
 198911b


## [v0.8.0](https://github.com/hoodie/icalendar-rs/compare/v0.7.0...v0.8.0) (2020-01-12)


### Features

* implement FromIter for Calendar
 5e88b03


### Fixes

* remove ToString and implement fmt::Display instead
 b52fcc4


## [v0.7.0](https://github.com/hoodie/icalendar-rs/compare/v0.6.0...v0.7.0) (2019-11-24)


## [v0.6.0](https://github.com/hoodie/icalendar-rs/compare/v0.5.0...v0.6.0) (2019-03-09)


## [v0.5.0](https://github.com/hoodie/icalendar-rs/compare/v0.4.0...v0.5.0) (2018-12-21)


## v0.4.0 (2018-12-21)

