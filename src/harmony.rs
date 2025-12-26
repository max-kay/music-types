//! This module contains types representing pitches and intervals in traditional wester music
//! theory in 12EDO

use std::{error::Error, fmt};

mod interval;
mod pitch;
pub mod scale;
pub use interval::{ChromaticInterval, ChromaticOctave, Interval, Octave, ParseIntervalError};
pub use pitch::{Accidental, ChromaticPitch, ParsePitchError, Pitch, PitchName};

#[derive(Debug)]
/// Error which can occur during parsing
pub enum ParseError {
    /// An Error from trying to parse a Pitch
    Pitch(ParsePitchError),
    /// An Error from trying to parse an Interval
    Interval(ParseIntervalError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Pitch(e) => e.fmt(f),
            ParseError::Interval(e) => e.fmt(f),
        }
    }
}

impl Error for ParseError {}

impl From<ParsePitchError> for ParseError {
    fn from(value: ParsePitchError) -> Self {
        Self::Pitch(value)
    }
}

impl From<ParseIntervalError> for ParseError {
    fn from(value: ParseIntervalError) -> Self {
        Self::Interval(value)
    }
}
