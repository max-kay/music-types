// #![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// Types representing pitch and intervals
pub mod pitch;

pub use pitch::{Interval, IntervalClass, Pitch, PitchClass};

/// Types representing scales and pitches
pub mod scale;
