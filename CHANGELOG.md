# Changelog

### [v0.13.3](https://github.com/hoodie/icalendar-rs/compare/v0.13.2...v0.13.3) (2022-10-25)

#### Fixes

* disable oldtime feature of chrono
  ([de14149](https://github.com/hoodie/icalendar-rs/commit/de14149f69a0666825712bceca031fdebd8a1524))

### [v0.13.2](https://github.com/hoodie/icalendar-rs/compare/v0.13.1...v0.13.2) (2022-10-18)

#### Features

* make Calendar components and properties public
  ([badbce2](https://github.com/hoodie/icalendar-rs/commit/badbce214c4497cf71206f2aef38021059bd12f4))
* add Component::find_prop helper
  ([cb10674](https://github.com/hoodie/icalendar-rs/commit/cb106742a3db54295d069de6c4985e20ba96f4f6))

### [v0.13.1](https://github.com/hoodie/icalendar-rs/compare/v0.13.0...v0.13.1) (2022-08-06)

#### Features

* convert parsed properties into DatePerhapsTime
  ([c0ffee9](https://github.com/hoodie/icalendar-rs/commit/c0ffee994521862f2c270aaf6b42e4c1c69556a3))

#### Fixes

* update uuid to 1.0
  ([c27ac42](https://github.com/hoodie/icalendar-rs/commit/c27ac4272e357ac03ef55e07cdefb9a58c58d04b))

## [v0.13.0](https://github.com/hoodie/icalendar-rs/compare/v0.12.1...v0.13.0) (2022-04-20)

### ⚠ BREAKING CHANGE

* Adds a new variant to `CalendarDateTime` enum.* Changes the type of `Component::all_day`.* Removes `Component::start_date` and`Component::end_date`.* Changes the type of `Component::starts` and`Component::ends`.* Changes the type of `Component::timestamp` and`Component::get_timestamp`.

### Features

* Add support for date-time with timezone.
  ([e914853](https://github.com/hoodie/icalendar-rs/commit/e9148536b571171a7f8fc7fa9eacb25b3f06be39))
* Add getters for Parameter key and value.
  ([472f4ca](https://github.com/hoodie/icalendar-rs/commit/472f4cac70060e8321b58664f6b4ef4d93ebb97e))
* Accept Into<DatePerhapsTime> for starts and ends.
  ([91b1d7d](https://github.com/hoodie/icalendar-rs/commit/91b1d7dab17d0b306ac7258be9bc06e8b0a3c022))

### Fixes

* Take NaiveDate for all_day.
  ([193c3cf](https://github.com/hoodie/icalendar-rs/commit/193c3cfa8f9dc5efa92eccd9082f66c53a655af7))
* Remove start_date and end_date.
  ([c0c67fb](https://github.com/hoodie/icalendar-rs/commit/c0c67fbd4301fa04fa82fc8e1df9dc49e4190420))
* According to RFC5545, DTSTAMP must be in UTC.
  ([9d2fb43](https://github.com/hoodie/icalendar-rs/commit/9d2fb4330e7c72f7f253af16401d8a1ba34c2d45))

### [v0.12.1](https://github.com/hoodie/icalendar-rs/compare/v0.12.0...v0.12.1) (2022-04-15)

#### Features

* Add getter and setter for URL property.
  ([eb53e1a](https://github.com/hoodie/icalendar-rs/commit/eb53e1a11f84dac29381c4c06cca33fa41b2d11c))

#### Fixes

* Export DatePerhapsTime type.
  ([8a46f0c](https://github.com/hoodie/icalendar-rs/commit/8a46f0c4e8b33929514eb05c141595f4c23599cc))
* Fix typo in README.
  ([7d117b2](https://github.com/hoodie/icalendar-rs/commit/7d117b2789cc853ed396104b99a6ac7b1580a06c))

## [v0.12.0](https://github.com/hoodie/icalendar-rs/compare/v0.11.1...v0.12.0) (2022-04-13)

### Features

* Allow due date (without time) to be specified for TODO.
  ([2a4a451](https://github.com/hoodie/icalendar-rs/commit/2a4a451a570ebb1e0de942aae5feb4dd5564c062))
* Add getters for date-time properties.
  ([9619316](https://github.com/hoodie/icalendar-rs/commit/96193160328249255f9230ef118f643321cc900d))
* Add getter for ValueType on Property.
  ([293dc70](https://github.com/hoodie/icalendar-rs/commit/293dc70f631b62767113082dbbc9d2653f3bc50d))
* Add getters for calendar properties.
  ([c97acaa](https://github.com/hoodie/icalendar-rs/commit/c97acaaa366814320b207f315a7e79e4a9330f17))
* Add getter for property parameters.
  ([3702a20](https://github.com/hoodie/icalendar-rs/commit/3702a20fe186f5abe5422bc75ea14a90561464cf))
* Add getters for basic todo and event properties.
  ([dad79f4](https://github.com/hoodie/icalendar-rs/commit/dad79f4589f9cd4eaa1743ce4b55f8a89690f2c7))
* Add getters for venue properties.
  ([174947a](https://github.com/hoodie/icalendar-rs/commit/174947a495699e4e3146a2f7034b64df0f7bddc5))
* Add getters for basic properties.
  ([437770b](https://github.com/hoodie/icalendar-rs/commit/437770b2e81edee67a57bf9dec4616b7643656c2))
* derive clone and equality traits.
  ([5d7a8a9](https://github.com/hoodie/icalendar-rs/commit/5d7a8a9747a8700afcf697edc89307f42833c13b))

### [v0.11.1](https://github.com/hoodie/icalendar-rs/compare/v0.11.0...v0.11.1) (2022-04-03)

#### Fixes

* build docs for parser feature
  ([9c2db74](https://github.com/hoodie/icalendar-rs/commit/9c2db748221fdc578a4c3809c54b0325a189f35e))

## [v0.11.0](https://github.com/hoodie/icalendar-rs/compare/v0.10.0...v0.11.0) (2022-01-24)

### Features

* add timestamp setter
  ([c0ffee1](https://github.com/hoodie/icalendar-rs/commit/c0ffee17ec48334c55d4cdbda7a02113ac6c8e9d))
* add serde support
  ([c0ffee7](https://github.com/hoodie/icalendar-rs/commit/c0ffee7ea499786897b94d5c433a77698fcd45a9))
* add try_into_string to Component
  ([c0ffeea](https://github.com/hoodie/icalendar-rs/commit/c0ffeead7f62a7589391c15456edb9cde68ebf46))
* deprecate fmt_write
  ([c0ffee0](https://github.com/hoodie/icalendar-rs/commit/c0ffee0dcb0be442c2dff6415d73cbafaf264e3d))
* add TryInto<String> for &Calendar
  ([c0ffeed](https://github.com/hoodie/icalendar-rs/commit/c0ffeed3ea1c3a371eb97c55f5e2f5273e819908))
* add FromStr implementations
  ([c0ffee2](https://github.com/hoodie/icalendar-rs/commit/c0ffee25ed0c18f4ff5a1049aea3368f1ba05dc0))
* expose `CalendarElement`
  ([684deda](https://github.com/hoodie/icalendar-rs/commit/684dedaf3b2551ef5e74264825ddbc4e4e184489))
* add missing .done() for `Calendar`
  ([7bbde0e](https://github.com/hoodie/icalendar-rs/commit/7bbde0e5a6d2d7ec8ed94e91fdde5bb80736aacd))
* more From implementations for Calendar
  ([70c70c1](https://github.com/hoodie/icalendar-rs/commit/70c70c121512a3c193f642fb3895eea0505a401a))
* add FromStr implementation for calendar
  ([e670469](https://github.com/hoodie/icalendar-rs/commit/e670469242ec3c4bc6a6b26a2bad65d2264b4267))
* reserialize parsed calendar
  ([64c116d](https://github.com/hoodie/icalendar-rs/commit/64c116d104511542ddfaaf38101eb55bed836e9c))
* simple nom parser
  ([21c0cfa](https://github.com/hoodie/icalendar-rs/commit/21c0cfa2a660fc83ef91fd506c2dc5f6669d8aad))

### Fixes

* unfold by tabs as well
  ([c0ffee8](https://github.com/hoodie/icalendar-rs/commit/c0ffee8b480396470ea25bd2354b62bc503d320e))
* lost spaces when folding
  ([c0ffeea](https://github.com/hoodie/icalendar-rs/commit/c0ffeeab611d46bd71fb75ba85b755df5a8a98b8))
* conversion between parsed Property and Property lost params
  ([b71826d](https://github.com/hoodie/icalendar-rs/commit/b71826dcdb783e3928ff00757d63d457dbdd3910))

## [v0.10.0](https://github.com/hoodie/icalendar-rs/compare/v0.9.0...v0.10.0) (2021-02-12)

### Features

* add propery value method
  ([c300630](https://github.com/hoodie/icalendar-rs/commit/c300630d91718414e02d3aa3354108abcff78d53))

## [v0.9.0](https://github.com/hoodie/icalendar-rs/compare/v0.8.0...v0.9.0) (2020-02-25)

### Features

* Support calendar properties
  ([1c1ed6b](https://github.com/hoodie/icalendar-rs/commit/1c1ed6ba62f4f023bf47ca9ae96c0f7b3689685a))
* Implement iCalendar Venue Draft
  ([198911b](https://github.com/hoodie/icalendar-rs/commit/198911bb821f8c464b8a2019ea5092a097a695b6))

## [v0.8.0](https://github.com/hoodie/icalendar-rs/compare/v0.7.0...v0.8.0) (2020-01-12)

### Features

* implement FromIter for Calendar
  ([5e88b03](https://github.com/hoodie/icalendar-rs/commit/5e88b0350e77b38a829b84e684b752b21d6b759a))

### Fixes

* remove ToString and implement fmt::Display instead
  ([b52fcc4](https://github.com/hoodie/icalendar-rs/commit/b52fcc4fb81ba08faee6f4f9b479a71dca6dc853))

## [v0.7.0](https://github.com/hoodie/icalendar-rs/compare/v0.6.0...v0.7.0) (2019-11-24)

## [v0.6.0](https://github.com/hoodie/icalendar-rs/compare/v0.5.0...v0.6.0) (2019-03-09)

## [v0.5.0](https://github.com/hoodie/icalendar-rs/compare/v0.4.0...v0.5.0) (2018-12-21)

## v0.4.0 (2018-12-21)
