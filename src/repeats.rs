//! Ways to repeat an event
//!
//! Taken from the rfc:
//!
//! ```
//!   +----------+--------+--------+-------+-------+------+-------+------+
//!   |          |SECONDLY|MINUTELY|HOURLY |DAILY  |WEEKLY|MONTHLY|YEARLY|
//!   +----------+--------+--------+-------+-------+------+-------+------+
//!   |BYMONTH   |Limit   |Limit   |Limit  |Limit  |Limit |Limit  |Expand|
//!   +----------+--------+--------+-------+-------+------+-------+------+
//!   |BYWEEKNO  |N/A     |N/A     |N/A    |N/A    |N/A   |N/A    |Expand|
//!   +----------+--------+--------+-------+-------+------+-------+------+
//!   |BYYEARDAY |Limit   |Limit   |Limit  |N/A    |N/A   |N/A    |Expand|
//!   +----------+--------+--------+-------+-------+------+-------+------+
//!   |BYMONTHDAY|Limit   |Limit   |Limit  |Limit  |N/A   |Expand |Expand|
//!   +----------+--------+--------+-------+-------+------+-------+------+
//!   |BYDAY     |Limit   |Limit   |Limit  |Limit  |Expand|Note 1 |Note 2|
//!   +----------+--------+--------+-------+-------+------+-------+------+
//!   |BYHOUR    |Limit   |Limit   |Limit  |Expand |Expand|Expand |Expand|
//!   +----------+--------+--------+-------+-------+------+-------+------+
//!   |BYMINUTE  |Limit   |Limit   |Expand |Expand |Expand|Expand |Expand|
//!   +----------+--------+--------+-------+-------+------+-------+------+
//!   |BYSECOND  |Limit   |Expand  |Expand |Expand |Expand|Expand |Expand|
//!   +----------+--------+--------+-------+-------+------+-------+------+
//!   |BYSETPOS  |Limit   |Limit   |Limit  |Limit  |Limit |Limit  |Limit |
//!   +----------+--------+--------+-------+-------+------+-------+------+
//! ```


//     RRULE:FREQ=YEARLY;BYMONTH=11;BYDAY=1SU
//     RRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=2SU

use chrono::{DateTime, Local};

//#[derive(Debug,Clone)]
//pub struct Every {
//    freq: Freq,
//    limit: Vec<Limit>,
//    interval: usize,
//    recurrence: Recurrence
//}
//
//impl Every {
//    pub fn new() -> Self {
//        Every {
//            freq: None,
//            limit: Vec::new(),
//            interval: 0,
//            recurrence: None
//        }
//    }
//    pub fn second(&mut self) -> &mut Self {
//        self.interval = 2;
//        self
//    }
//}

#[derive(Debug,Clone)]
pub enum Freq {
    Daily,
    Hourly,
    Minutely,
    Monthly,
    Weekly,
    Yearly,
    Secondly,
    None
}

#[derive(Debug,Clone)]
pub enum Limit {
    Until(DateTime<Local>),
    Count(usize),
    Interval(usize)
}

#[derive(Debug,Clone)]
pub enum Recurrence {
    //Bysecond( Byseclist )
    //Byminute( Byminlist )
    //Byhour( Byhrlist )
    //Byday( Bywdaylist )
    //Bymonthday( Bymodaylist )
    //Byyearday( Byyrdaylist )
    //Byweekno( Bywknolist )
    //Bymonth( Bymolist )
    //Bysetpos( Bysplist )
    //Wkst( Weekday ),
    None
}


enum Weekday {
    Su, Mo, Tu, We, Th, Fr, Sa
}










pub trait Repeater: Sized {}
