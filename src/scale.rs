use crate::{Interval, IntervalClass, Pitch, PitchClass};

mod display;
mod parse;

mod standard_scales;
#[derive(Clone, PartialEq, Eq, Debug)]
/// This struct represents a scale.
/// Here we take a scale to be a collection of interval classes sorted by the standard sorting of
/// intervals. see struct [`IntervalClass`] for more information.
/// Scales represented by this struct are allways normal.
/// A normal Scale is scale which starts with the unison interval and is sorted.
///
///
/// # FromStr implementation
/// For ease of creation this type implements FromStr.
/// The from str method expects a list of intervals separated by whitespace.
/// ```
/// # use music_types::scale::Scale;
/// use std::str::FromStr;
/// let major = Scale::from_str("1 j2 j3 4 5 j6 j7").unwrap();
/// assert_eq!(major, Scale::major());
/// ```
/// But if no interval modifier is present 2, 3 and 6 are assumed to be major and 7 is assumed to
/// be minor.
/// ```
/// # use music_types::scale::Scale;
/// use std::str::FromStr;
/// let major = Scale::from_str("1 2 3 4 5 6 j7").unwrap();
/// assert_eq!(major, Scale::major());
///
/// let mixolydian = Scale::from_str("1 2 3 4 5 6 7").unwrap();
/// assert_eq!(mixolydian, Scale::mixolydian());
/// ```
pub struct Scale(Vec<IntervalClass>);

impl Scale {
    /// Creates a new Scale.
    /// This function sorts the intervals and adds a unison at the start.
    /// This means a scale created from this function will allways be normal.
    pub fn new(mut intervals: Vec<IntervalClass>) -> Self {
        if !intervals.is_sorted() {
            intervals.sort();
        }
        if intervals[0] != IntervalClass::new(0, 0) {
            intervals.insert(0, IntervalClass::new(0, 0));
        }
        Self(intervals)
    }

    /// Creates a new Scale from the intervals doesn't check any assumptions.
    ///
    /// For more see [`Self::new`]
    pub fn new_unchecked(intervals: Vec<IntervalClass>) -> Self {
        Self(intervals)
    }

    /// return true if the Scale is normal
    /// For more see type level docs.
    pub fn is_normal(&self) -> bool {
        self.0.is_sorted() && self.0[0] == IntervalClass::new(0, 0)
    }

    /// panics if the scale is not normal
    pub fn next_mode(&self) -> Self {
        if !self.is_normal() {
            panic!("nonnormal scale was used in next mode");
        }
        self.nth_mode(1)
    }

    /// panics if the scale is not normal
    pub fn nth_mode(&self, n: u32) -> Self {
        let n = n as usize % self.0.len();
        if !self.is_normal() {
            panic!("nonnormal scale was used in nth mode");
        }
        if self.0.len() == 1 {
            return self.clone();
        }
        let interval = self.0[n];
        let mut new_intervals: Vec<_> = self.0.iter().map(|i| i - interval).collect();
        new_intervals.sort();
        Self(new_intervals)
    }
}

impl Scale {
    /// Creates a rooted scale from the intervals, takes ownership of self.
    pub fn to_rooted(self, root: PitchClass) -> RootedScale {
        RootedScale {
            root,
            intervals: self.0,
        }
    }

    /// Creates a rooted scale from the intervals, doesn't take ownership of self.
    pub fn add_root(&self, root: PitchClass) -> RootedScale {
        RootedScale {
            root,
            intervals: self.0.clone(),
        }
    }
}

pub struct RootedScale {
    root: PitchClass,
    intervals: Vec<IntervalClass>,
}

pub struct ScaleIter<'a> {
    root: Pitch,
    octave: i8,
    index: usize,
    intervals: &'a [IntervalClass],
}

impl Iterator for ScaleIter<'_> {
    type Item = Pitch;

    fn next(&mut self) -> Option<Self::Item> {
        let item = Some(self.root + self.intervals[self.index].place_in_octave(self.octave));
        self.index += 1;
        if self.index >= self.intervals.len() {
            self.octave += 1;
            self.index = 0;
        }
        item
    }
}

impl RootedScale {
    pub fn iter_from_octave<'a>(&'a self, octave: i8) -> ScaleIter<'a> {
        ScaleIter {
            root: self.root.place_in_octave(octave),
            octave: 0,
            index: 0,
            intervals: &*self.intervals,
        }
    }
}
