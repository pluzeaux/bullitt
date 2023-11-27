use core::fmt;
use std::fmt::Display;
use time::{Duration, Time};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Clock {
    time: Time
}

impl Clock {
    pub fn new(hours: i64, minutes:i64) -> Self {
        Self {
           // time: time!(0:00:00) + Duration::hours(hours) + Duration::minutes(minutes)
           time: Time::from_hms(0, 0, 0).unwrap() + Duration::hours(hours) + Duration::minutes(minutes)
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Self {
            time: self.time + Duration::minutes(i64::from(minutes))
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.time.hour(), self.time.minute())
    }
}

#[test]
fn on_the_hour() {
    assert_eq!(Clock::new(8, 0).to_string(), "08:00");
}

#[test]
fn hours_and_minutes_roll_over_continuously() {
    assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
}

#[test]
fn compare_clocks_for_equality() {
    assert_eq!(Clock::new(15, 37), Clock::new(15, 37));
}

#[test]
fn compare_clocks_with_negative_hour_that_wraps() {
    assert_eq!(Clock::new(-31, 3), Clock::new(17, 3));
}