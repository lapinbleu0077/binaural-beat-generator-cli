//! A module that contains code related to the duration functionality.

use std::fmt;

use crate::modules::duration::duration_common::ToMinutes;

/// Represents common durations in minutes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Duration {
    FiveMinutes,
    TenMinutes,
    FifteenMinutes,
    TwentyMinutes,
    ThirtyMinutes,
    ThirtyFiveMinutes,
    FortyMinutes,
    FiftyMinutes,
    SixtyMinutes,
}

/// This formatter will return the number of minutes for the given duration enum.
impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Duration::FiveMinutes => write!(f, "5 min"),
            Duration::TenMinutes => write!(f, "10 min"),
            Duration::FifteenMinutes => write!(f, "15 min"),
            Duration::TwentyMinutes => write!(f, "20 min"),
            Duration::ThirtyMinutes => write!(f, "30 min"),
            Duration::ThirtyFiveMinutes => write!(f, "35 min"),
            Duration::FortyMinutes => write!(f, "40 min"),
            Duration::FiftyMinutes => write!(f, "50 min"),
            Duration::SixtyMinutes => write!(f, "60 min"),
        }
    }
}

/// This implementation will convert the duration type of minutes into integer minutes.
impl ToMinutes for Duration {
    fn to_minutes(&self) -> u32 {
        match self {
            Duration::FiveMinutes => 5,
            Duration::TenMinutes => 10,
            Duration::FifteenMinutes => 15,
            Duration::TwentyMinutes => 20,
            Duration::ThirtyMinutes => 30,
            Duration::ThirtyFiveMinutes => 35,
            Duration::FortyMinutes => 40,
            Duration::FiftyMinutes => 50,
            Duration::SixtyMinutes => 60,
        }
    }
}

/// This function will return the a vector list of all the supported durations.
pub fn duration_list() -> Vec<Duration> {
    return vec![
        Duration::FiveMinutes,
        Duration::TenMinutes,
        Duration::FifteenMinutes,
        Duration::TwentyMinutes,
        Duration::ThirtyMinutes,
        Duration::ThirtyFiveMinutes,
        Duration::FortyMinutes,
        Duration::FiftyMinutes,
        Duration::SixtyMinutes,
    ];
}
