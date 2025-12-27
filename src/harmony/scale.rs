//! this module contains types representing scales
use std::ops::Add;

use crate::{
    div_remainder,
    harmony::{Accidental, ChromaticOctave, Interval, Pitch},
};

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
            intervals.sort_by(Interval::cmp_chromatic);
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
        let mut new_intervals: Vec<_> = self
            .0
            .iter()
            .map(|i| (i - interval) % ChromaticOctave)
            .collect();
        new_intervals.sort_by(Interval::cmp_chromatic);
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

/// An accidental in a keysignature
///
/// This accidental is independent of the octave
/// In german "Vorzeichen"
#[derive(Debug, Clone, Copy)]
pub struct KeyAccidental {
    staffposition: i16,
    accidental: Accidental,
}

impl KeyAccidental {
    /// creates a accidental
    pub fn new(staffposition: i16, accidental: Accidental) -> Self {
        let (_octave, staffposition) = div_remainder(staffposition, 7);
        Self {
            staffposition,
            accidental,
        }
    }
}

/// An accidental on a staffline
///
/// This accidental is only valid for one octave.
/// In german "Versetzungszeichen"
#[derive(Debug, Clone, Copy)]
pub struct ConcreteAccidental {
    staffposition: i16,
    accidental: Accidental,
}

impl ConcreteAccidental {
    /// creates a conrete accidental on the given staffposition
    pub fn new(staffposition: i16, accidental: Accidental) -> Self {
        Self {
            staffposition,
            accidental,
        }
    }
}

/// A KeySignature
#[derive(Debug, Clone, Default)]
pub struct KeySignature(Vec<KeyAccidental>);

impl KeySignature {
    /// creates the keysignature of the major scale with root `pitch`
    pub fn major(pitch: Pitch) -> Self {
        // TODO: proper sorting of the accidentals so the are listed in canonical order
        let mut accs = Vec::new();
        for p in Scale::major().iter_from_root(pitch).take(7) {
            accs.push(KeyAccidental::new(p.staff_position(), p.accidental()))
        }
        Self(accs)
    }

    /// creates the keysignature of the minor scale with root `pitch`
    pub fn minor(pitch: Pitch) -> Self {
        Self::major(pitch + Interval::MIN_THIRD)
    }
}

/// Calculates the accidental that needs to be displayed in the context of a keysignature and
/// preceding accidentals
///
/// # Example
/// ```
/// # use music_types::harmony::{scale::{KeySignature, AccidentalCalulator}, Pitch, ParsePitchError, Accidental};
/// # use std::str::FromStr;
/// // create calculator with key signature Bb
/// let key = KeySignature::major(Pitch::class_from_str("Bb")?);
/// let mut calculator: AccidentalCalulator = key.into();
///
/// // no accidental needed because Eb is in the key of Bb
/// assert_eq!(calculator.get_and_update(Pitch::from_str("Eb4")?), None);
/// // a flat needed because Ab is not in the key of Bb
/// assert_eq!(calculator.get_and_update(Pitch::from_str("Ab4")?), Some(Accidental::FLAT));
/// // flat not needed anymore because Ab was updated before
/// assert_eq!(calculator.get_and_update(Pitch::from_str("Ab4")?), None);
/// // flat needed because Ab5 is in a different octave than Ab4
/// assert_eq!(calculator.get_and_update(Pitch::from_str("Ab5")?), Some(Accidental::FLAT));
///
/// // clear the accidental stack, for example after a barline is encountered
/// calculator.clear();
///
/// // the key signature persists after clearing the accidental stack
/// assert_eq!(calculator.get_and_update(Pitch::from_str("Eb3")?), None);
/// // but the Ab needs a flat again
/// assert_eq!(calculator.get_and_update(Pitch::from_str("Ab4")?), Some(Accidental::FLAT));
/// // a natural needed because Eb is in the key of Bb but E is not
/// assert_eq!(calculator.get_and_update(Pitch::from_str("E4")?), Some(Accidental::NATURAL));
/// // a flat needed because E natural is currently on stack
/// assert_eq!(calculator.get_and_update(Pitch::from_str("Eb4")?), Some(Accidental::FLAT));
///
/// // change the key signature to E minor
/// let key = KeySignature::minor(Pitch::from_str("E4")?);
/// calculator.change_key_signature(key);
/// // now F# doesn't need an accidental because its in the key of E minor
/// assert_eq!(calculator.get_and_update(Pitch::from_str("F#2")?), None);
/// # Ok::<(), ParsePitchError>(())
/// ```
#[derive(Debug, Clone, Default)]
pub struct AccidentalCalulator {
    signature: Vec<KeyAccidental>,
    accidentals: Vec<ConcreteAccidental>,
}

impl AccidentalCalulator {
    /// create an AccidentalCalculator from a key signature
    pub fn from_key_signature(key: KeySignature) -> Self {
        key.into()
    }
}

impl From<KeySignature> for AccidentalCalulator {
    fn from(value: KeySignature) -> Self {
        Self {
            signature: value.0,
            accidentals: Vec::new(),
        }
    }
}

impl AccidentalCalulator {
    /// gets the display accidental
    pub fn get_display_accidental(&self, pitch: Pitch) -> Option<Accidental> {
        for acc in self.accidentals.iter().rev() {
            if acc.staffposition == pitch.staff_position() {
                if acc.accidental != pitch.accidental() {
                    return Some(pitch.accidental());
                } else {
                    return None;
                }
            }
        }
        for acc in &self.signature {
            if acc.staffposition == pitch.staff_position().rem_euclid(7) {
                if acc.accidental != pitch.accidental() {
                    return Some(pitch.accidental());
                } else {
                    return None;
                }
            }
        }
        if pitch.accidental() != Accidental::NATURAL {
            return Some(pitch.accidental());
        } else {
            return None;
        }
    }

    /// gets the display accidental and updates the stack of accidentals as needed
    pub fn get_and_update(&mut self, pitch: Pitch) -> Option<Accidental> {
        let opt = self.get_display_accidental(pitch);
        if let Some(acc) = opt {
            self.push(ConcreteAccidental::new(pitch.staff_position(), acc))
        }
        opt
    }

    /// clears the accumulated accidental stack
    pub fn clear(&mut self) {
        self.accidentals.clear()
    }

    /// changes the key signature and clears the accidental stack
    pub fn change_key_signature(&mut self, key: KeySignature) {
        self.accidentals.clear();
        self.signature = key.0;
    }

    /// pushes the accidental on the current stack
    pub fn push(&mut self, accidental: ConcreteAccidental) {
        // should I remove accidentals the become unnecessary because they're overwritten?
        self.accidentals.push(accidental)
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
