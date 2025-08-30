//! A module that contains common code related to the duration functionality.
//! 

/// A trait to allow for associated enums to be converted to minutes.
pub trait ToMinutes {
    fn to_minutes(&self) -> u32;
}
