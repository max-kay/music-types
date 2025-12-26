use std::ops::{Add, Neg, Rem, Sub};

use crate::{
    div_remainder,
    harmony::pitch::{ChromaticPitch, Pitch},
};

mod display;
mod parse;

pub use parse::ParseIntervalError;

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

macro_rules! impl_assigning {
    ($t1:ty, $t2:ty) => {
        impl std::ops::AddAssign<$t2> for $t1 {
            fn add_assign(&mut self, rhs: $t2) {
                *self = *self + rhs;
            }
        }

        #[doc(hidden)]
        impl std::ops::AddAssign<&$t2> for $t1 {
            fn add_assign(&mut self, rhs: &$t2) {
                *self = *self + *rhs;
            }
        }

        impl std::ops::SubAssign<$t2> for $t1 {
            fn sub_assign(&mut self, rhs: $t2) {
                *self = *self - rhs;
            }
        }

        #[doc(hidden)]
        impl std::ops::SubAssign<&$t2> for $t1 {
            fn sub_assign(&mut self, rhs: &$t2) {
                *self = *self - *rhs;
            }
        }
    };

    ($t:ty) => {
        impl_assigning!($t, $t);
    };
}

macro_rules! rem_assign {
    ($t1:ty, $t2:ty) => {
        impl std::ops::RemAssign<$t2> for $t1 {
            fn rem_assign(&mut self, rhs: $t2) {
                *self = *self % rhs
            }
        }

        #[doc(hidden)]
        impl std::ops::RemAssign<&$t2> for $t1 {
            fn rem_assign(&mut self, rhs: &$t2) {
                *self = *self % *rhs
            }
        }
    };
}

macro_rules! complete_group {
    ($t:ty) => {
        #[doc(hidden)]
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
        impl_assigning!($t);
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
        impl_assigning!($element, $group);
    };
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A type representing an interval
///
/// # Creating an interval
/// Since the representation of interval is not intuitive even for someone familiar with music
/// theory, it is suggested to use the implementation of [`FromStr`][`std::str::FromStr`] to create Intervals.
/// The interval consists of an optional `-`, the interval quality and the interval number.
///
/// The interval quality is parse from:
/// - `d` dimished
/// - `m` minor
/// - `p` or `P` perfect
/// - `j` or `M` major
/// - `a` or `A` augmented
///
/// Additionaly, constants for the most common intervals exist.
///
/// # Examples
/// ```
/// # use music_types::harmony::{Interval, ParseIntervalError};
/// # use std::str::FromStr;
/// assert_eq!(Interval::from_str("1")?, Interval::UNISON);
/// assert_eq!(Interval::from_str("p1")?, Interval::UNISON);
/// assert_eq!(Interval::from_str("m3")?, Interval::MIN_THIRD);
/// assert_eq!(Interval::from_str("j3")?, Interval::MAJ_THIRD);
/// assert_eq!(Interval::from_str("M3")?, Interval::MAJ_THIRD);
/// assert_eq!(Interval::from_str("d5")?, Interval::DIM_FIFTH);
/// // for a major third down
/// assert_eq!(Interval::from_str("-j3")?, -Interval::MAJ_THIRD);
/// # Ok::<(), ParseIntervalError>(())
/// ```
///
/// To allow the representation of any interval quality parenthases are used.
/// The interval quality is parse from:
/// - `(-2)` dimished
/// - `(-1)` minor
/// - `(0)` perfect
/// - `(1)` or `(+1)` major
/// - `(2)` or `(+2)` augmented
///
/// Note that for intervals which have perfect quality `(-1)` and `(1)` cannot be used.
/// Simmilarly for intervals which have minor and major quality where `(0)` cannot be used.
///
/// # Examples
/// ```
/// # use music_types::harmony::{Interval, ParseIntervalError};
/// # use std::str::FromStr;
/// assert_eq!(Interval::from_str("m3")?, Interval::from_str("(-1)3")?);
/// assert_eq!(Interval::from_str("j3")?, Interval::from_str("(+1)3")?);
/// assert_eq!(Interval::from_str("4")?, Interval::from_str("(0)4")?);
/// assert_eq!(Interval::from_str("a4")?, Interval::from_str("(+2)4")?);
/// # Ok::<(), ParseIntervalError>(())
/// ```
///
/// When trying to parse an invalid combination of interval number ParseIntervalError::Impossible
/// is returned.
///
/// # Examples
/// ```
/// # use music_types::harmony::{Interval, ParseIntervalError};
/// # use std::str::FromStr;
/// assert!(matches!(
///     Interval::from_str("m8"),
///     Err(ParseIntervalError::Impossible{number: 8, quality: _ }),
/// ));
/// ```
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
    /// Creates a new interval from the steps.
    pub fn new(chromatic_steps: i16, diatonic_steps: i16) -> Self {
        Self {
            chromatic: chromatic_steps,
            diatonic: diatonic_steps,
        }
    }

    fn has_perfect(diatonic_steps: i16) -> bool {
        match diatonic_steps.rem_euclid(7) {
            0 | 3 | 4 => true,
            1 | 2 | 5 | 6 => false,
            (i16::MIN..0) | (7..) => unreachable!("interval number must be less than 7"),
        }
    }

    fn to_chromatic_steps_minor(diatonic_steps: i16) -> i16 {
        if diatonic_steps < 0 {
            return -Self::to_chromatic_steps_minor(-diatonic_steps);
        }
        let (octave, d_steps) = div_remainder(diatonic_steps, 7);
        return octave * 12
            + match d_steps {
                1 => 1,  // second
                2 => 3,  // third
                5 => 8,  // sixth
                6 => 10, // sevents
                0 | 3 | 4 => panic!(
                    "tried to calculate diatonic steps to minor interval for interval which cannot have minor interval quality"
                ),
                (i16::MIN..0) | (7..) => unreachable!(),
            };
    }

    fn to_chromatic_steps_perfect(diatonic_steps: i16) -> i16 {
        if diatonic_steps < 0 {
            return -Self::to_chromatic_steps_perfect(-diatonic_steps);
        }

        let (octave, d_steps) = div_remainder(diatonic_steps, 7);
        return octave * 12
            + match d_steps {
                0 => 0, // unison
                3 => 5, // fourth
                4 => 7, // fifth
                1 | 2 | 5 | 6 => panic!(
                    "tried to caluclate diatonic steps to perfect for interval which cannot have perfect interval quality"
                ),
                (i16::MIN..0) | (7..) => unreachable!(),
            };
    }
}

#[allow(missing_docs)]
/// constant for intervals in the first octave
impl Interval {
    pub const UNISON: Self = Interval {
        diatonic: 0,
        chromatic: 0,
    };

    pub const MIN_SECOND: Self = Interval {
        diatonic: 1,
        chromatic: 1,
    };
    pub const MAJ_SECOND: Self = Interval {
        diatonic: 1,
        chromatic: 2,
    };

    pub const MIN_THIRD: Self = Interval {
        diatonic: 2,
        chromatic: 3,
    };
    pub const MAJ_THIRD: Self = Interval {
        diatonic: 2,
        chromatic: 4,
    };

    pub const FOURTH: Self = Interval {
        diatonic: 3,
        chromatic: 5,
    };
    pub const AUG_FOURTH: Self = Interval {
        diatonic: 3,
        chromatic: 6,
    };
    pub const DIM_FIFTH: Self = Interval {
        diatonic: 4,
        chromatic: 6,
    };
    pub const FIFTH: Self = Interval {
        diatonic: 4,
        chromatic: 7,
    };

    pub const MIN_SIXTH: Self = Interval {
        diatonic: 5,
        chromatic: 8,
    };
    pub const MAJ_SIXTH: Self = Interval {
        diatonic: 5,
        chromatic: 9,
    };

    pub const MIN_SEVENTH: Self = Interval {
        diatonic: 6,
        chromatic: 10,
    };
    pub const MAJ_SEVENTH: Self = Interval {
        diatonic: 6,
        chromatic: 11,
    };

    pub const OCTAVE: Self = Interval {
        diatonic: 7,
        chromatic: 12,
    };
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

#[derive(Copy, Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// Useful for Rem operations.
///
/// After taking the rem of a pitch or interval it ensures that the diatonic part of the object is
/// in 0..=6
/// For Pitch this is equivalent to setting the octave to 4.
///
/// # Examples
/// ```
/// # use std::str::FromStr;
/// # use music_types::harmony::{Pitch, Interval, Octave};
/// assert_eq!(Pitch::from_str("F6")? % Octave, Pitch::from_str("F4")?);
/// assert_eq!(Interval::from_str("m13")? % Octave, Interval::from_str("m6")?);
/// assert_eq!(Interval::from_str("-m3")? % Octave, Interval::from_str("j6")?);
/// # Ok::<(), music_types::harmony::ParseError>(())
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Octave;

impl Rem<Octave> for Pitch {
    type Output = Pitch;

    fn rem(self, _rhs: Octave) -> Self::Output {
        let (octave, diatonic) = div_remainder(self.diatonic, 7);
        Self {
            diatonic,
            chromatic: self.chromatic - octave * 12,
        }
    }
}
rem_assign!(Pitch, Octave);

impl Rem<Octave> for Interval {
    type Output = Interval;

    fn rem(self, _rhs: Octave) -> Self::Output {
        let (octave, diatonic) = div_remainder(self.diatonic, 7);
        Self {
            diatonic,
            chromatic: self.chromatic - octave * 12,
        }
    }
}
rem_assign!(Interval, Octave);

/// Useful for Rem operations.
///
/// After taking the rem of a pitch or interval it ensures that the chromatic part of the object is
/// in 0..=12
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChromaticOctave;

impl Rem<ChromaticOctave> for Pitch {
    type Output = Pitch;

    fn rem(self, _rhs: ChromaticOctave) -> Self::Output {
        let (octave, chromatic) = div_remainder(self.chromatic, 12);
        Self {
            diatonic: self.diatonic - octave * 7,
            chromatic,
        }
    }
}
rem_assign!(Pitch, ChromaticOctave);

impl Rem<ChromaticOctave> for ChromaticPitch {
    type Output = ChromaticPitch;

    fn rem(self, _rhs: ChromaticOctave) -> Self::Output {
        let (_octave, chromatic) = div_remainder(self.0, 12);
        Self(chromatic)
    }
}
rem_assign!(ChromaticPitch, ChromaticOctave);

impl Rem<ChromaticOctave> for Interval {
    type Output = Interval;

    fn rem(self, _rhs: ChromaticOctave) -> Self::Output {
        let (octave, chromatic) = div_remainder(self.chromatic, 12);
        Self {
            diatonic: self.diatonic - octave * 7,
            chromatic,
        }
    }
}
rem_assign!(Interval, ChromaticOctave);

impl Rem<ChromaticOctave> for ChromaticInterval {
    type Output = ChromaticInterval;

    fn rem(self, _rhs: ChromaticOctave) -> Self::Output {
        let (_octave, chromatic) = div_remainder(self.0, 12);
        Self(chromatic)
    }
}
rem_assign!(ChromaticInterval, ChromaticOctave);

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
        transpose!("Bb4", "m3", "Db5");
        transpose!("Bb4", "j3", "D5");
        transpose!("Bb4", "-j3", "Gb4");
    }

    fn t_i(p1: &str, p2: &str, i: &str) {
        let res = Pitch::from_str(p2).unwrap() - Pitch::from_str(p1).unwrap();
        assert_eq!(
            res,
            Interval::from_str(i).unwrap(),
            "{p2} - {p1} evaluated to {res}"
        )
    }
    #[test]
    fn to_interval() {
        t_i("C4", "E4", "j3");
        t_i("C4", "G4", "5");
        t_i("G4", "C4", "-5");
    }
}
