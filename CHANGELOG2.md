# Changelog
## [[0.15.6](https://github.com/hoodie/icalendar-rs/compare/v0.15.5...v0.15.6)] – 2023-07-22


### Bug Fixes

- *(deps)* update rust crate chrono-tz to 0.8.3 ([1a31f9a](1a31f9a0bad05905db2e381b6e592f44cb0ee817))


## [[0.15.5](https://github.com/hoodie/icalendar-rs/compare/v0.15.4...v0.15.5)] – 2023-07-21


### Documentation


-  fix build badge ([1268fa4](1268fa46689c58ea298a76901545d5ffb4d9e1fb))
### Features


-  add wasm support ([336dee7](336dee785da9a9763c854731893f6877a763a9c2))

## [[0.15.4](https://github.com/hoodie/icalendar-rs/compare/v0.15.3...v0.15.4)] – 2023-02-17


### Features


-  update iso8601 ([92f8329](92f8329c839bc9dd8d376c0f81306a524807556a))

## [[0.15.3](https://github.com/hoodie/icalendar-rs/compare/v0.15.2...v0.15.3)] – 2023-02-12


### Features


-  add better api and examples for dates with timezones ([6245b27](6245b27d6d291d982cf705526d91a4dede8ebe02))
-  create start and end from naivedatetime+timezone tuple ([0d9eda8](0d9eda8d839e646a4b12691c57fb4d6c23a1cdd8))
-  raise MSRV to 1.60 ([c434a85](c434a85065f901d39083c0f965cb8cd390bda0e4))

## [[0.15.2](https://github.com/hoodie/icalendar-rs/compare/v0.15.1...v0.15.2)] – 2023-02-01


### Features


-  derive clone for several types ([c62d679](c62d679234d74c33958d3a0aafa46fcd5e0cd73f))

## [[0.15.1](https://github.com/hoodie/icalendar-rs/compare/v0.15.0...v0.15.1)] – 2023-01-02


### Bug Fixes


-  add default calendar properties in From impl ([ba44631](ba44631fda4bacd8b104acc57de2b01eb384429a))
-  do not duplicate default properties when parsing ([d6b42d8](d6b42d86fe63d2e33b41672978bb34853387b418))

## [[0.15.0](https://github.com/hoodie/icalendar-rs/compare/v0.14.2...v0.15.0)] – 2022-12-25


### Bug Fixes


-  retain properties in parsed calendar ([9403dbf](9403dbf4c651206c3bef691f5b5b43a3bbe28d2d)), *fixes #61*
### Features


-  [**breaking**] add basic properties to calendar in default() instead at serialization time" ([f065258](f0652583a73509c7fa0646588628a81bc9f92cfa)), *BREAKING CHANGE:Calendar::default() will now contain basic properties (VERSION, CALSCALE, PRODID) which were before added during serialization.*

## [[0.14.2](https://github.com/hoodie/icalendar-rs/compare/v0.14.1...v0.14.2)] – 2022-12-15


### Features


-  Make Calendar::property_value() publicly accessible ([ec6ae2f](ec6ae2f0580a01e1e27476bb3e379697de7e872d))

## [[0.14.1](https://github.com/hoodie/icalendar-rs/compare/v0.14.0...v0.14.1)] – 2022-12-14


### Bug Fixes


-  remove pretty_assertions dependency ([eb501d9](eb501d9e9c12bec30ca20b63169a2dbeb74cab38))

## [[0.14.0](https://github.com/hoodie/icalendar-rs/compare/v0.13.3...v0.14.0)] – 2022-11-20


### Documentation


-  add alarm struct documentation ([c280de5](c280de5e3339eaf7a5042a342afab86014612156))
### Features


-  add From<Date> impl to DatePerhapsTime ([30b041a](30b041a0d83d60ad8653651f857d4b04f56e822c))
-  add support for sequence properties ([ce57466](ce57466ca091490be4cff53c869b01943f901487))
-  add support for Alarms ([eb4a1bd](eb4a1bd9c7b012869e139c03fee39337e3d69930))
-  [**breaking**] add support for Alarm component ([0b04255](0b04255098a927bd035946deec7b505d07799356)), *<https://www.rfc-editor.org/rfc/rfc5545.html#section-3.6.6>

* supporting Alarms with action = "DISPLAY" and "AUDIO" only so far*, *BREAKING CHANGE:pulled certain component methods into `EventLike` trait*

## [[0.13.3](https://github.com/hoodie/icalendar-rs/compare/v0.13.2...v0.13.3)] – 2022-10-25


### Bug Fixes


-  disable oldtime feature of chrono ([de14149](de14149f69a0666825712bceca031fdebd8a1524))

## [[0.13.2](https://github.com/hoodie/icalendar-rs/compare/v0.13.1...v0.13.2)] – 2022-10-18


### Features


-  add Component::find_prop helper ([cb10674](cb106742a3db54295d069de6c4985e20ba96f4f6))
-  make Calendar components and properties public ([badbce2](badbce214c4497cf71206f2aef38021059bd12f4))

## [[0.13.1](https://github.com/hoodie/icalendar-rs/compare/v0.13.0...v0.13.1)] – 2022-08-06


### Bug Fixes


-  update uuid to 1.0 ([c27ac42](c27ac4272e357ac03ef55e07cdefb9a58c58d04b))
### Documentation


-  update maintenance badge in readme ([84bec41](84bec41bbdbb17feabca87256ee2fdd001320576))
-  update head of crate documentation ([b2eb6b8](b2eb6b88b0e2fa27f31296184b29e80ca7886e4c))
-  publish docs for ParseString ([5d58303](5d58303de249283f0b8e751f9ba16aafc63d6c65))
### Features


-  convert parsed properties into DatePerhapsTime ([c0ffee9](c0ffee994521862f2c270aaf6b42e4c1c69556a3)), *Fixes:https://github.com/hoodie/icalendar-rs/issues/44*

## [[0.13.0](https://github.com/hoodie/icalendar-rs/compare/v0.12.1...v0.13.0)] – 2022-04-20


### Bug Fixes


-  [**breaking**] According to RFC5545, DTSTAMP must be in UTC. ([9d2fb43](9d2fb4330e7c72f7f253af16401d8a1ba34c2d45)), *BREAKING CHANGE:Changes the type of `Component::timestamp` and*, *`Component::get_timestamp`.*
-  [**breaking**] Remove start_date and end_date. ([c0c67fb](c0c67fbd4301fa04fa82fc8e1df9dc49e4190420)), *BREAKING CHANGE:Removes `Component::start_date` and*, *`Component::end_date`.*
-  [**breaking**] Take NaiveDate for all_day. ([193c3cf](193c3cfa8f9dc5efa92eccd9082f66c53a655af7)), *BREAKING CHANGE:Changes the type of `Component::all_day`.*
### Features


-  [**breaking**] Accept Into<DatePerhapsTime> for starts and ends. ([91b1d7d](91b1d7dab17d0b306ac7258be9bc06e8b0a3c022)), *BREAKING CHANGE:Changes the type of `Component::starts` and*, *`Component::ends`.*
-  Add getters for Parameter key and value. ([472f4ca](472f4cac70060e8321b58664f6b4ef4d93ebb97e))
-  [**breaking**] Add support for date-time with timezone. ([e914853](e9148536b571171a7f8fc7fa9eacb25b3f06be39)), *BREAKING CHANGE:Adds a new variant to `CalendarDateTime` enum.*

## [[0.12.1](https://github.com/hoodie/icalendar-rs/compare/v0.12.0...v0.12.1)] – 2022-04-15


### Bug Fixes


-  Fix typo in README. ([7d117b2](7d117b2789cc853ed396104b99a6ac7b1580a06c))
-  Export DatePerhapsTime type. ([8a46f0c](8a46f0c4e8b33929514eb05c141595f4c23599cc))
### Features


-  Add getter and setter for URL property. ([eb53e1a](eb53e1a11f84dac29381c4c06cca33fa41b2d11c))

## [[0.12.0](https://github.com/hoodie/icalendar-rs/compare/v0.11.1...v0.12.0)] – 2022-04-13


### Features


-  derive clone and equality traits. ([5d7a8a9](5d7a8a9747a8700afcf697edc89307f42833c13b))
-  Add getters for basic properties. ([437770b](437770b2e81edee67a57bf9dec4616b7643656c2))
-  Add getters for venue properties. ([174947a](174947a495699e4e3146a2f7034b64df0f7bddc5))
-  Add getters for basic todo and event properties. ([dad79f4](dad79f4589f9cd4eaa1743ce4b55f8a89690f2c7))
-  Add getter for property parameters. ([3702a20](3702a20fe186f5abe5422bc75ea14a90561464cf))
-  Add getters for calendar properties. ([c97acaa](c97acaaa366814320b207f315a7e79e4a9330f17))
-  Add getter for ValueType on Property. ([293dc70](293dc70f631b62767113082dbbc9d2653f3bc50d))
-  Add getters for date-time properties. ([9619316](96193160328249255f9230ef118f643321cc900d))
-  [**breaking**] Allow due date (without time) to be specified for TODO. ([2a4a451](2a4a451a570ebb1e0de942aae5feb4dd5564c062))

## [[0.11.1](https://github.com/hoodie/icalendar-rs/compare/v0.11.0...v0.11.1)] – 2022-04-03


### Bug Fixes


-  build docs for parser feature ([9c2db74](9c2db748221fdc578a4c3809c54b0325a189f35e))

## [[0.11.0](https://github.com/hoodie/icalendar-rs/compare/v0.10.0...v0.11.0)] – 2022-01-24


### Bug Fixes


-  conversion between parsed Property and Property lost params ([b71826d](b71826dcdb783e3928ff00757d63d457dbdd3910))
-  lost spaces when folding ([c0ffeea](c0ffeeab611d46bd71fb75ba85b755df5a8a98b8))
-  unfold by tabs as well ([c0ffee8](c0ffee8b480396470ea25bd2354b62bc503d320e))
### Documentation


-  link types in markdown documentation ([a884112](a884112667f68614004e70ae339c8d8018f13d9f))
-  add full circle example ([21a91a9](21a91a94e26b27c19ea2a493ed4dc85e2ae4029e))
-  add examples and links to Calendar ([cda9629](cda96296062d8de64de3f43a1ea4458f2ec8e9ac))
-  update crate description and keywords ([f934f29](f934f2942f6f6ff50dc1d2c2eb8bac31d57918d6))
-  overhaul README ([4664ba6](4664ba6fb8dbcf38a35e2d63172a440c8ab9d43f))
-  reference rfc in method documentation ([995ae3a](995ae3ad03de79475b6c79edc9919e438cf38754))
-  reference rfc in enum documentation ([92e6fe6](92e6fe656a3b7896ba521213e8515e3724f4aa83))
-  add examples ([e68842b](e68842bdc8a92c4c629eec98278872a5387697c3))
-  simplify examples ([c0ffee3](c0ffee39c01cbee727202a59b846b1897cd72f35))
-  overhaul readme ([c0ffee8](c0ffee8a0ed7b7a30cb39388e68d1ebec2f15b7c))
### Features


-  simple nom parser ([21c0cfa](21c0cfa2a660fc83ef91fd506c2dc5f6669d8aad))
-  reserialize parsed calendar ([64c116d](64c116d104511542ddfaaf38101eb55bed836e9c))
-  add FromStr implementation for calendar ([e670469](e670469242ec3c4bc6a6b26a2bad65d2264b4267))
-  more From implementations for Calendar ([70c70c1](70c70c121512a3c193f642fb3895eea0505a401a))
-  add missing .done() for `Calendar` ([7bbde0e](7bbde0e5a6d2d7ec8ed94e91fdde5bb80736aacd))
-  expose `CalendarElement` ([684deda](684dedaf3b2551ef5e74264825ddbc4e4e184489))
-  add FromStr implementations ([c0ffee2](c0ffee25ed0c18f4ff5a1049aea3368f1ba05dc0))
-  add TryInto<String> for &Calendar ([c0ffeed](c0ffeed3ea1c3a371eb97c55f5e2f5273e819908))
-  deprecate fmt_write ([c0ffee0](c0ffee0dcb0be442c2dff6415d73cbafaf264e3d))
-  add try_into_string to Component ([c0ffeea](c0ffeead7f62a7589391c15456edb9cde68ebf46))
-  add serde support ([c0ffee7](c0ffee7ea499786897b94d5c433a77698fcd45a9))
-  add timestamp setter ([c0ffee1](c0ffee17ec48334c55d4cdbda7a02113ac6c8e9d))

## [[0.10.0](https://github.com/hoodie/icalendar-rs/compare/v0.9.0...v0.10.0)] – 2021-02-12


### Features


-  [**breaking**] add propery value method ([c300630](c300630d91718414e02d3aa3354108abcff78d53)), *BREAKING CHANGE:Propery's key method now returns the more conventional &str instead of a cloned String*

## [[0.9.0](https://github.com/hoodie/icalendar-rs/compare/v0.8.0...v0.9.0)] – 2020-02-25


### Features


-  Implement iCalendar Venue Draft ([198911b](198911bb821f8c464b8a2019ea5092a097a695b6))
-  Support calendar properties ([1c1ed6b](1c1ed6ba62f4f023bf47ca9ae96c0f7b3689685a)), *Adds:- name
- description
- timezone
- ttl*

## [[0.8.0](https://github.com/hoodie/icalendar-rs/compare/v0.7.0...v0.8.0)] – 2020-01-12


### Bug Fixes


-  remove ToString and implement fmt::Display instead ([b52fcc4](b52fcc4fb81ba08faee6f4f9b479a71dca6dc853))
### Features


-  implement FromIter for Calendar ([5e88b03](5e88b0350e77b38a829b84e684b752b21d6b759a))

