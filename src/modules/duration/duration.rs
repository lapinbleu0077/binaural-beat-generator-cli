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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_duration_enum_to_integer_minutes_cases {
        ($($name:ident:($a:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(Duration::to_minutes($a),$expected)
                }
            )*
        };
    }

    macro_rules! test_duration_enum_to_text_minutes_cases {
        ($($name:ident:($a:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($a,$expected)
                }
            )*
        };
    }

    #[test]
    fn duration_list_has_exact_items() {
        let existing_list = duration_list();
        let expected_list = vec![
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

        assert_eq!(existing_list, expected_list);
    }

    #[test]
    fn duration_list_has_expected_sequence_for_items() {
        let existing_list = duration_list();
        let expected_list = vec![
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

        assert_eq!(existing_list.len(), expected_list.len());

        for (index, existing_value) in existing_list.iter().enumerate() {
            let expected_entry = expected_list.get(index).unwrap();
            assert_eq!(existing_value, expected_entry);
        }
    }

    test_duration_enum_to_integer_minutes_cases! {
        five_minutes_integer: (&Duration::FiveMinutes,5),
        ten_minutes_integer: (&Duration::TenMinutes,10),
        fifteen_minutes_integer: (&Duration::FifteenMinutes,15),
        twenty_minutes_integer: (&Duration::TwentyMinutes,20),
        thirty_minutes_integer: (&Duration::ThirtyMinutes,30),
        thirtyfive_minutes_integer: (&Duration::ThirtyFiveMinutes,35),
        forty_minutes_integer: (&Duration::FortyMinutes,40),
        fifty_minutes_integer: (&Duration::FiftyMinutes,50),
        sixty_minutes_integer: (&Duration::SixtyMinutes,60),
    }

    test_duration_enum_to_text_minutes_cases! {
        five_minutes_text: (Duration::FiveMinutes.to_string(),"5 min"),
        ten_minutes_text: (Duration::TenMinutes.to_string(),"10 min"),
        fifteen_minutes_text: (Duration::FifteenMinutes.to_string(),"15 min"),
        twenty_minutes_text: (Duration::TwentyMinutes.to_string(),"20 min"),
        thirty_minutes_text: (Duration::ThirtyMinutes.to_string(),"30 min"),
        thirtyfive_minutes_text: (Duration::ThirtyFiveMinutes.to_string(),"35 min"),
        forty_minutes_text: (Duration::FortyMinutes.to_string(),"40 min"),
        fifty_minutes_text: (Duration::FiftyMinutes.to_string(),"50 min"),
        sixty_minutes_text: (Duration::SixtyMinutes.to_string(),"60 min"),
    }
}
