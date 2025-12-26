//! this module contains types representing scales
use std::ops::Add;

use crate::harmony::{ChromaticOctave, Interval, Octave, Pitch};

mod display;
mod parse;

mod standard_scales;

#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// This struct represents a scale.
/// Here we take a scale to be a collection of interval classes sorted by the standard sorting of
/// intervals. see struct [`Interval`] for more information.
/// Scales represented by this struct are allways normal.
/// A normal Scale is scale which starts with the unison interval and is sorted.
///
///
/// # FromStr implementation
/// For ease of creation this type implements FromStr.
/// The from str method expects a list of intervals separated by whitespace.
/// ```
/// # use music_types::harmony::{scale::Scale, ParseError};
/// use std::str::FromStr;
/// let major = Scale::from_str("1 j2 j3 4 5 j6 j7")?;
/// assert_eq!(major, Scale::major());
/// # Ok::<(), ParseError>(())
/// ```
/// But if no interval quality is present 2, 3 and 6 are assumed to be major and 7 is assumed to
/// be minor.
/// ```
/// # use music_types::harmony::{scale::Scale, ParseError};
/// use std::str::FromStr;
/// let major = Scale::from_str("1 2 3 4 5 6 j7")?;
/// assert_eq!(major, Scale::major());
///
/// let mixolydian = Scale::from_str("1 2 3 4 5 6 7")?;
/// assert_eq!(mixolydian, Scale::mixolydian());
/// # Ok::<(), ParseError>(())
/// ```
pub struct Scale(Vec<Interval>);

impl Scale {
    /// Creates a new Scale.
    /// This function sorts the intervals, adds a unison at the start and places them in the first
    /// octave.
    pub fn new(mut intervals: Vec<Interval>) -> Self {
        intervals.iter_mut().for_each(|i| *i %= ChromaticOctave);
        if !intervals.is_sorted() {
            intervals.sort();
        }
        if intervals[0] != Interval::new(0, 0) {
            intervals.insert(0, Interval::new(0, 0));
        }
        Self(intervals)
    }

    /// return true if the Scale is normal
    /// For more see type level docs.
    fn is_normal(&self) -> bool {
        self.0.is_sorted() && self.0[0] == Interval::new(0, 0)
    }

    /// returns the next mode of the scale
    ///
    /// # Example
    /// ```
    /// # use music_types::harmony::scale::Scale;
    /// assert_eq!(Scale::major().next_mode(), Scale::dorian());
    /// ```
    pub fn next_mode(&self) -> Self {
        if !self.is_normal() {
            panic!("nonnormal scale was used in next mode");
        }
        self.nth_mode(1)
    }

    /// returns the nth mode of the scale
    ///
    /// Note that zero indexing is used
    ///
    /// # Examples
    /// ```
    /// # use music_types::harmony::scale::Scale;
    /// assert_eq!(Scale::major().nth_mode(0), Scale::ionian());
    /// assert_eq!(Scale::major().nth_mode(1), Scale::dorian());
    /// assert_eq!(Scale::major().nth_mode(2), Scale::phrygian());
    /// assert_eq!(Scale::major().nth_mode(3), Scale::lydian());
    /// assert_eq!(Scale::major().nth_mode(4), Scale::mixolydian());
    /// assert_eq!(Scale::major().nth_mode(5), Scale::aeolian());
    /// assert_eq!(Scale::major().nth_mode(6), Scale::locrian());
    /// assert_eq!(Scale::major().nth_mode(7), Scale::ionian());
    /// ```
    pub fn nth_mode(&self, n: u32) -> Self {
        let n = n as usize % self.0.len();
        if !self.is_normal() {
            panic!("nonnormal scale was used in nth mode");
        }
        if self.0.len() == 1 {
            return self.clone();
        }
        let interval = self.0[n];
        let mut new_intervals: Vec<_> = self.0.iter().map(|i| (i - interval) % Octave).collect();
        new_intervals.sort();
        Self(new_intervals)
    }

    /// retruns an infinite iterator over the intervals of the scale
    pub fn iter<'a>(&'a self) -> ScaleIter<'a, Interval> {
        ScaleIter {
            root: Interval::UNISON,
            index: 0,
            intervals: &self.0,
        }
    }

    /// retruns an infinite iterator over the pitches of the scales starting from root
    pub fn iter_from_root<'a>(&'a self, root: Pitch) -> ScaleIter<'a, Pitch> {
        ScaleIter {
            root,
            index: 0,
            intervals: &self.0,
        }
    }
}

#[derive(Debug)]
/// an infinite iterator over a scale
pub struct ScaleIter<'a, T> {
    root: T,
    index: usize,
    intervals: &'a [Interval],
}

impl<T: Add<Interval, Output = T> + Copy> Iterator for ScaleIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = Some(self.root + self.intervals[self.index]);
        self.index += 1;
        if self.index >= self.intervals.len() {
            self.root = self.root + Interval::OCTAVE;
            self.index = 0;
        }
        item
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    macro_rules! check_next {
        ($iter:ident, $pitch:literal) => {
            assert_eq!(($iter).next().unwrap(), Pitch::from_str($pitch).unwrap());
        };
    }

    use super::*;
    #[test]
    fn major_scale_test() {
        let major = Scale::major();
        let mut iter = major.iter_from_root(Pitch::from_str("C4").unwrap());
        check_next!(iter, "C4");
        check_next!(iter, "D4");
        check_next!(iter, "E4");
        check_next!(iter, "F4");
        check_next!(iter, "G4");
        check_next!(iter, "A4");
        check_next!(iter, "B4");
        check_next!(iter, "C5");
        check_next!(iter, "D5");
    }
}
