//! A module that contains common code related to the frequency functionality.

/// Trait for types that can provide a frequency in Hz (f32).
pub trait ToFrequency {
    fn to_hz(&self) -> f32;
}
