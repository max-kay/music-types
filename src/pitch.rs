mod display;
mod parse;

use std::ops::{Add, Neg, Sub};
use std::panic;

pub use parse::{IntervalParseError, PitchParseError};
///!
///! About the repesentation
///! Accidentals to notes are a complicated system of displacement from their diatonic
///! representation. Even more complicated is the representation of interval as some intervals like
///! unison have perfect versions but no minor or major versions and some have minor and minor
///! versions but no perfect version.
///! This library solves this problem by representing pitches and intervals by diatonic steps and
///! chromatic steps.
///! Diatonic steps are equivalent to staff spaces. So the diatonic steps of an interval which shifts a pitch by one staff
///! space for example C to D is 1.
///! Chromatic steps are more self explanatory it just counts the chromatic steps which are taken
///! from one note to another. So the chromatic steps of the interval from C to D are 2.
///! Note the change from one indexing to zero indexing in the diatonic steps.
///! A major second has the representation diatonic: 1 chromatic: 2.
///! A minor second has the representation diatonic: 1 chromatic: 1.
///!
///! Pitches can now be expressed as intervals to middle c (C4 in scientific pitch notation).
///! This makes the transpostition of notes very simple. When the name of a note is needed the pitch
///! name and the accidental can then be calculated from this representation.
///! This way all transpositions can be represented correctly, but allow for arbitrary number of
///! flats and sharps on any note.
///!
///! To reduce the complexity for the user of this crate FromStr implementations for all types which
///! face this problem are implemented using representations familliar to a user knowing music
///! theory.

/// returns a, b such that a*y + b = x and 0 <= b < y
fn div_remainder(x: i16, y: i16) -> (i16, i16) {
    let q = x / y;
    let m = x % y;
    if m < 0 {
        return if y > 0 {
            (q - 1, m + y)
        } else {
            (q + 1, m + y)
        };
    }
    (q, m)
}

macro_rules! impl_op_for_refs {
    ($t:ty, $trait:ident, $method:ident) => {
        impl_op_for_refs!($t, $t, $trait, $method);
    };

    ($tl:ty, $tr:ty, $trait:ident, $method:ident) => {
        #[doc(hidden)]
        impl std::ops::$trait<$tr> for &$tl {
            type Output = <$tl as std::ops::$trait<$tr>>::Output;

            fn $method(self, rhs: $tr) -> Self::Output {
                (*self).$method(rhs)
            }
        }

        #[doc(hidden)]
        impl std::ops::$trait<&$tr> for $tl {
            type Output = <$tl as std::ops::$trait<$tr>>::Output;

            fn $method(self, rhs: &$tr) -> Self::Output {
                self.$method(*rhs)
            }
        }

        #[doc(hidden)]
        impl std::ops::$trait<&$tr> for &$tl {
            type Output = <$tl as std::ops::$trait<$tr>>::Output;

            fn $method(self, rhs: &$tr) -> Self::Output {
                (*self).$method(*rhs)
            }
        }
    };
}

macro_rules! complete_group {
    ($t:ty) => {
        impl std::ops::Neg for &$t {
            type Output = $t;
            fn neg(self) -> Self::Output {
                -*self
            }
        }
        impl std::ops::Sub for $t {
            type Output = $t;

            fn sub(self, rhs: $t) -> Self::Output {
                self + (-rhs)
            }
        }

        impl_op_for_refs!($t, Add, add);
        impl_op_for_refs!($t, Sub, sub);
    };
}

macro_rules! complete_action {
    ($group:ty, $element:ty) => {
        impl std::ops::Add<$element> for $group {
            type Output = $element;
            fn add(self, rhs: $element) -> Self::Output {
                rhs.add(self)
            }
        }
        impl std::ops::Sub<$group> for $element {
            type Output = $element;
            fn sub(self, rhs: $group) -> Self::Output {
                self + (-rhs)
            }
        }
        impl_op_for_refs!($element, Sub, sub);
        impl_op_for_refs!($element, $group, Add, add);
        impl_op_for_refs!($group, $element, Add, add);
        impl_op_for_refs!($element, $group, Sub, sub);
    };
}

#[derive(Copy, Clone, PartialEq, Eq)]
/// This struct represents an accidental.
/// It can be created using FromStr. "#" and "b" are used to represent sharps and flats
/// respectively.
///
/// "+" and "&" are used to represent double sharps and double flats respectively, any higher
/// composition of flats and sharps are represented using repetition of their respective symbols or
/// a number followed by "#" or "b" in parenthases. (3#) or (2b)
/// "##" and "bb" are valid too.
pub struct Accidental(i16);

impl Default for Accidental {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// Pitch name is a wrapper around char ensuring the value is one of C D E F G A B
pub struct PitchName(u8);

impl Default for PitchName {
    fn default() -> Self {
        Self('C' as u8)
    }
}

impl PitchName {
    /// Return a PitchName if the given char is a valid note name
    pub fn new(c: char) -> Option<Self> {
        match c {
            'A'..='G' => Some(Self(c as u8)),
            _ => None,
        }
    }

    /// Creates the pitch according to this number of diatonic steps. C represented as 0 + 7*n.
    pub fn from_diatonic_steps(diatonic: impl Into<i16>) -> Self {
        let pitch = diatonic.into().rem_euclid(7);
        Self(match pitch {
            0 => 'C',
            1 => 'D',
            2 => 'E',
            3 => 'F',
            4 => 'G',
            5 => 'A',
            6 => 'B',
            _ => unreachable!(),
        } as u8)
    }

    /// Return the diatonic steps to C
    pub const fn to_diatonic_steps(self) -> u8 {
        match self.0 as char {
            'C' => 0,
            'D' => 1,
            'E' => 2,
            'F' => 3,
            'G' => 4,
            'A' => 5,
            'B' => 6,
            _ => unreachable!(),
        }
    }

    /// Returns the chromatic steps from C to the natural pitch
    pub fn to_chromatic_steps(self) -> u8 {
        match self.to_diatonic_steps() {
            0 => 0,
            1 => 2,
            2 => 4,
            3 => 5,
            4 => 7,
            5 => 9,
            6 => 11,
            _ => unreachable!(),
        }
    }

    /// Returns the char for this pitch class
    pub fn as_char(self) -> char {
        self.0 as char
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
/// A class representing a Pitch.
pub struct Pitch {
    chromatic: i16,
    diatonic: i16,
}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.chromatic.partial_cmp(&other.chromatic) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.diatonic.partial_cmp(&other.diatonic)
    }
}

impl Ord for Pitch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Pitch {
    /// This function decomposes the pitch into its parts in terms of scientific pitch notation
    /// where middle c is the start of the 4th octave.
    pub fn decompose(self) -> (PitchName, Accidental, i16) {
        let (octave, note) = div_remainder(self.diatonic, 7);
        let diatonic_name = PitchName::from_diatonic_steps(note);
        let chromatic_natural = octave * 12 + diatonic_name.to_chromatic_steps() as i16;
        (
            diatonic_name,
            Accidental(self.chromatic - chromatic_natural),
            octave + 4,
        )
    }

    /// This function composes a pitch from the parts of its name in scientific pitch notation
    /// where middle c is the start of the 4th octave.
    pub fn compose(name: PitchName, accidental: Accidental, octave: i16) -> Self {
        let note = name.to_diatonic_steps() as i16;
        let offset = name.to_chromatic_steps() as i16;
        Self {
            diatonic: (octave - 4) * 7 + note,
            chromatic: (octave - 4) * 12 + offset + accidental.0,
        }
    }
}

impl Pitch {
    /// Converts to the chromatic pitch
    pub fn to_chromatic(self) -> ChromaticPitch {
        self.into()
    }

    /// Converts to the pitch class
    pub fn to_class(self) -> PitchClass {
        self.into()
    }

    /// Converts to the chromatic pitch class
    pub fn to_chromatic_class(self) -> ChromaticPitchClass {
        self.into()
    }
}
impl From<Pitch> for PitchClass {
    fn from(value: Pitch) -> Self {
        let (octave, note) = div_remainder(value.diatonic, 7);
        Self {
            diatonic: note as u8,
            chromatic: (value.chromatic - octave * 12) as i8,
        }
    }
}

impl From<Pitch> for ChromaticPitch {
    fn from(value: Pitch) -> Self {
        Self(value.chromatic)
    }
}

impl From<Pitch> for ChromaticPitchClass {
    fn from(value: Pitch) -> Self {
        Self(value.chromatic.rem_euclid(12) as u8)
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
/// This struct represents a pitch class. A pitch class is a pitch where the octave is not given.
pub struct PitchClass {
    diatonic: u8,
    chromatic: i8,
}

impl PitchClass {
    /// Decomposes the pitch class into a its pitch name and accidental
    pub fn decompose(self) -> (PitchName, Accidental) {
        let name = PitchName::from_diatonic_steps(self.diatonic);
        let chromatic_natural = name.to_chromatic_steps() as i16;
        (name, Accidental(self.chromatic as i16 - chromatic_natural))
    }

    /// Creates a pitch class from the pitch name and accidental
    pub fn compose(name: PitchName, acc: Accidental) -> Self {
        let diatonic = name.to_diatonic_steps();
        let chromatic = (name.to_chromatic_steps() as i16 + acc.0) as i8;
        Self {
            diatonic,
            chromatic,
        }
    }

    /// Places the pitch into an octave.
    /// The octave is given according to scientific pitch notation where middle c is the start of
    /// the 4th octave.
    pub fn place_in_octave(self, octave: i8) -> Pitch {
        Pitch {
            diatonic: self.diatonic as i16 + (octave - 4) as i16 * 7,
            chromatic: self.chromatic as i16 + (octave - 4) as i16 * 12,
        }
    }

    /// Converts to the chromatic pitch class
    pub fn to_chromatic(self) -> ChromaticPitchClass {
        self.into()
    }
}

impl From<PitchClass> for ChromaticPitchClass {
    fn from(value: PitchClass) -> Self {
        Self(value.chromatic.rem_euclid(12) as u8)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
/// This struct represents an Interval
pub struct Interval {
    pub(crate) chromatic: i16,
    pub(crate) diatonic: i16,
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.chromatic.partial_cmp(&other.chromatic) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.diatonic.partial_cmp(&other.diatonic)
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            chromatic: 0,
            diatonic: 0,
        }
    }
}

impl Interval {
    fn has_perfect(mut diatonic_steps: i16) -> bool {
        diatonic_steps = diatonic_steps.abs();
        match diatonic_steps % 7 {
            0 | 3 | 4 => true,
            1 | 2 | 5 | 6 => false,
            (7..) => panic!("interval number must be less than 7"),
            (i16::MIN..0) => unreachable!(),
        }
    }

    fn to_diatonic_steps_minor(diatonic_steps: i16) -> i16 {
        assert!(
            !Self::has_perfect(diatonic_steps),
            "tried to calculate diatonic steps to minor interval for interval which doesn't have a minor version"
        );
        if diatonic_steps < 0 {
            return -Self::to_diatonic_steps_minor(-diatonic_steps);
        }
        let (octave, d_steps) = div_remainder(diatonic_steps, 7);
        return octave * 12
            + match d_steps {
                1 => 1,
                2 => 3,
                5 => 8,
                6 => 10,
                (7..) => panic!("interval number must be less than 7"),
                0 | 3 | 4 => unreachable!("checked above"),
                (i16::MIN..0) => unreachable!(),
            };
    }

    fn to_diatonic_steps_perfect(diatonic_steps: i16) -> i16 {
        assert!(
            Self::has_perfect(diatonic_steps),
            "tried to calculate diatonic steps to perfect interval for interval which doesn't have a perfect version"
        );
        if diatonic_steps < 0 {
            return -Self::to_diatonic_steps_perfect(-diatonic_steps);
        }

        let (octave, d_steps) = div_remainder(diatonic_steps, 7);
        return octave * 12
            + match d_steps {
                0 => 0,
                3 => 5,
                4 => 7,
                (7..) => panic!("interval number must be less than 7"),
                1 | 2 | 5 | 6 => unreachable!("checked above"),
                (i16::MIN..0) => unreachable!(),
            };
    }

    /// Creates a new interval from the steps.
    pub fn new(chromatic_steps: i16, diatonic_steps: i16) -> Self {
        Self {
            chromatic: chromatic_steps,
            diatonic: diatonic_steps,
        }
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            chromatic: self.chromatic + rhs.chromatic,
            diatonic: self.diatonic + rhs.diatonic,
        }
    }
}

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            chromatic: -self.chromatic,
            diatonic: -self.diatonic,
        }
    }
}
complete_group!(Interval);

impl Add<Interval> for Pitch {
    type Output = Pitch;

    fn add(self, rhs: Interval) -> Self::Output {
        Pitch {
            diatonic: self.diatonic + rhs.diatonic,
            chromatic: self.chromatic + rhs.chromatic,
        }
    }
}

impl Sub for Pitch {
    type Output = Interval;

    fn sub(self, rhs: Self) -> Self::Output {
        Interval {
            chromatic: self.chromatic - rhs.chromatic,
            diatonic: self.diatonic - rhs.diatonic,
        }
    }
}

complete_action!(Interval, Pitch);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Represents an interval independent of octave
pub struct IntervalClass(pub(crate) Interval);

impl IntervalClass {
    /// Creates a interval from the given steps by transposing by octaves into the first octave.
    pub fn new(diatonic_steps: i16, chromatic_steps: i16) -> Self {
        let this = Self(Interval::new(chromatic_steps, diatonic_steps));
        this.reduce()
    }

    /// Places the interval in an octave.
    ///
    /// # Examples
    /// ```
    /// # use music_types::pitch::{IntervalClass, Interval, IntervalParseError};
    /// # use std::str::FromStr;
    /// assert_eq!(IntervalClass::from_str("1")?.place_in_octave(0), Interval::from_str("1")?);
    /// assert_eq!(IntervalClass::from_str("j3")?.place_in_octave(1), Interval::from_str("j10")?);
    /// assert_eq!(IntervalClass::from_str("j6")?.place_in_octave(-1), Interval::from_str("-m3")?);
    /// # Ok::<(), IntervalParseError>(())
    /// ```
    pub fn place_in_octave(&self, octave: i8) -> Interval {
        let mut this = self.0;
        this.diatonic += 7 * octave as i16;
        this.chromatic += 12 * octave as i16;
        this
    }

    fn reduce(self) -> Self {
        let (octave, diatonic) = div_remainder(self.0.diatonic, 7);
        let chromatic = self.0.chromatic - 12 * octave;
        if chromatic < 0 {
            Self(Interval {
                diatonic: diatonic + 7,
                chromatic: self.0.chromatic - 12 * (octave - 1),
            })
        } else {
            Self(Interval {
                diatonic,
                chromatic: self.0.chromatic - 12 * octave,
            })
        }
    }
}

impl Add for IntervalClass {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0).reduce()
    }
}

impl Neg for IntervalClass {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0).reduce()
    }
}

complete_group!(IntervalClass);

#[derive(Copy, Clone, Default, Debug)]
/// Represents a chromatic pitch
pub struct ChromaticPitch(i16);

impl ChromaticPitch {
    /// Converts the pitch to a frequency using the standard tuning A4 = 440Hz
    pub fn to_frequency(self) -> f32 {
        self.to_frequency_tuning(440.0)
    }

    /// Converts the pitch to a frequency using the given tuning for A4
    pub fn to_frequency_tuning(self, a_4: f32) -> f32 {
        a_4 * 2.0_f32.powf((self.0 - 9) as f32 / 12.0)
    }

    /// Returns the midi pitch if it is with in the midi range
    /// The returned u8 is in 0..=127
    pub fn to_midi_pitch(self) -> Option<u8> {
        // my 0 (middle c) is 60 in MIDI
        let pitch = self.0 - 60;
        if (0..=127).contains(&pitch) {
            Some(pitch as u8)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
/// Represents a chromatic pitch class, a chromatic pitch independent of octave.
pub struct ChromaticPitchClass(u8);

impl ChromaticPitchClass {
    /// Creates a pitch class from the given chromatic pitch by
    /// reducing the pitch mod 12.
    pub fn new(num: u8) -> Self {
        Self(num.rem_euclid(12))
    }

    /// Places the pitch into an octave using the convention from scientific pitch notation that
    /// middle c is the start of the 4th octave.
    pub fn place_in_octave(self, octave: i16) -> ChromaticPitch {
        ChromaticPitch(self.0 as i16 + (octave - 4) * 12)
    }
}

#[derive(Copy, Clone, Default, Debug)]
/// Represents a chromatic interval, an interval counted in half steps.
pub struct ChromaticInterval(i16);

impl Add for ChromaticInterval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Neg for ChromaticInterval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

complete_group!(ChromaticInterval);

impl Add<ChromaticInterval> for ChromaticPitch {
    type Output = ChromaticPitch;

    fn add(self, rhs: ChromaticInterval) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for ChromaticPitch {
    type Output = ChromaticInterval;

    fn sub(self, rhs: Self) -> Self::Output {
        ChromaticInterval(self.0 - rhs.0)
    }
}
complete_action!(ChromaticInterval, ChromaticPitch);

/// A type representing a chromatic interval class, an interval counted in half steps ignoring the
/// octave.
pub struct ChromaticIntervalClass(u8);

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    macro_rules! neg {
        ($s:literal) => {
            assert_eq!(
                Interval::from_str($s).unwrap() + (-Interval::from_str($s).unwrap()),
                Interval::from_str("1").unwrap()
            )
        };
    }
    #[test]
    fn neg() {
        neg!("1");
        neg!("m2");
        neg!("5");
        neg!("m13");
    }

    macro_rules! i_add {
        ($i1:literal, $i2:literal, $res:literal) => {
            assert_eq!(
                Interval::from_str($i1).unwrap() + (Interval::from_str($i2).unwrap()),
                Interval::from_str($res).unwrap()
            )
        };
    }
    #[test]
    fn interval_add() {
        i_add!("1", "j3", "j3");
        i_add!("1", "5", "5");
        i_add!("1", "a5", "a5");
        i_add!("j3", "m3", "5");
        i_add!("m3", "j3", "5");
        i_add!("m3", "m3", "d5");
        i_add!("j3", "j3", "a5");
        i_add!("j2", "j3", "a4");
        i_add!("8", "j2", "j9");
        i_add!("5", "5", "j9");
        i_add!("5", "-5", "1");
        i_add!("5", "-m3", "j3");
        i_add!("j9", "-m3", "j7");
    }

    macro_rules! transpose {
        ($p:literal, $i:literal, $res:literal) => {
            let res = Interval::from_str($i).unwrap() + Pitch::from_str($p).unwrap();
            assert_eq!(
                res,
                Pitch::from_str($res).unwrap(),
                "{} + {} evaluated to {res}",
                $p,
                $i
            )
        };
    }

    #[test]
    fn transpose() {
        transpose!("C4", "j3", "E4");
        transpose!("C4", "m3", "Eb4");
        transpose!("C4", "j13", "A5");
        transpose!("C4", "m13", "Ab5");
        transpose!("C4", "m13", "Ab5");
    }

    fn t_i(p1: &str, p2: &str, i: &str) {
        let res = Pitch::from_str(p2).unwrap() - Pitch::from_str(p1).unwrap();
        assert_eq!(
            res,
            Interval::from_str(i).unwrap(),
            "{p2} - {p1} evalueated to {res}"
        )
    }
    #[test]
    fn to_interval() {
        t_i("C4", "E4", "j3");
        t_i("C4", "G4", "5");
        t_i("G4", "C4", "-5");
    }

    #[test]
    fn div() {
        assert_eq!((0_i16, 1), div_remainder(1, 3));
        assert_eq!((-1_i16, 2), div_remainder(-1, 3));
        assert_eq!((-2_i16, 1), div_remainder(-5, 3));
    }
}
