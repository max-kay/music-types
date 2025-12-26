mod display;
mod parse;

pub use parse::ParsePitchError;

use crate::div_remainder;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A accidental: flats, naturals, sharps, and higher order accidentals.
///
/// # FromStr implementation
///
/// Flats are parsed from `"b"`, sharps from `"#"` and naturals from the empty str `""` and from `"n"`.
/// `"+"` and `"&"` are used to represent double sharps and double flats respectively.
///
/// Additionally the unicode symbols `'\u{266d}'`(♭), `'\u{266e}'`(♮), `'\u{266f}'`(♯), `'\u{1d12a}'` (𝄫)
/// and `'\u{1d12b}'`(𝄪) are recognized.
///
/// Any higher
/// composition of flats and sharps are represented using repetition of their respective symbols or
/// a number followed by `"#"` or `"b"` in parenthases, e.g. `"(3#)"` = `"###"` and `"(3b)"` = `"bbb"`.
pub struct Accidental(i16);

impl Accidental {
    /// Constructs a accidental from the chromatic shift
    /// # Examples
    /// ```
    /// # use music_types::harmony::{Accidental, ParsePitchError};
    /// # use std::str::FromStr;
    /// assert_eq!(Accidental::new(1), Accidental::from_str("#")?);
    /// assert_eq!(Accidental::new(0), Accidental::from_str("")?);
    /// assert_eq!(Accidental::new(-1), Accidental::from_str("b")?);
    /// # Ok::<(), ParsePitchError>(())
    /// ```
    pub const fn new(chromatic_shift: i16) -> Self {
        Self(chromatic_shift)
    }

    /// Converts the accidental to the utf-8 aequivalent, if it exists.
    ///
    /// Note that '♭', '♮' and '♯' are in the unicode block for miscellaneous symbols (U+2600–U+26FF),
    /// but double flats '𝄫' and sharps '𝄪' are in the unicode block for musical symbols (U+1D100-U+1D1FF)
    /// and their implementation in fonts is rarer.
    ///
    /// Accidentals of higher order do not appear in the standard unicode table, for these `None`
    /// is returned
    pub fn to_utf8(&self) -> Option<char> {
        match self.0 {
            -1 => Some('\u{266d}'),
            0 => Some('\u{266e}'),
            1 => Some('\u{266f}'),
            2 => Some('\u{1d12a}'),
            -2 => Some('\u{1d12b}'),
            _ => None,
        }
    }
}

#[cfg(feature = "smufl")]
impl Accidental {
    /// returns the corresponding smufl glyph upto triple sharps and flats
    pub fn to_smufl(&self) -> Option<smufl::Glyph> {
        use smufl::Glyph::*;

        match self.0 {
            -3 => Some(AccidentalTripleFlat),
            -2 => Some(AccidentalDoubleFlat),
            -1 => Some(AccidentalFlat),
            0 => Some(AccidentalNatural),
            1 => Some(AccidentalSharp),
            2 => Some(AccidentalDoubleSharp),
            3 => Some(AccidentalTripleSharp),
            _ => None,
        }
    }
}

impl Default for Accidental {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Pitch name is a wrapper around char ensuring the value is one of C D E F G A B
pub struct PitchName(u8);

impl Default for PitchName {
    fn default() -> Self {
        Self(b'C')
    }
}

impl PartialOrd for PitchName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PitchName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // this implementation sorts pitchName to C D E F G A B
        let (mut a, mut b) = (self.0, other.0);
        if a < b'C' {
            a += b'H' - b'A';
        }
        if b < b'C' {
            b += b'H' - b'A';
        }
        a.cmp(&b)
    }
}

impl PitchName {
    /// Returns a PitchName if the given char is a valid note name
    ///
    /// expects uppercase letter
    pub fn new(c: char) -> Option<Self> {
        match c {
            'A'..='G' => Some(Self(c as u8)),
            _ => None,
        }
    }

    /// Return a PitchName if the given byte char is a valid note name
    ///
    /// expects uppercase letter
    pub fn from_byte(c: u8) -> Option<Self> {
        match c {
            b'A'..=b'G' => Some(Self(c as u8)),
            _ => None,
        }
    }

    /// Creates the pitch according to this number of diatonic steps. C represented as 0 + 7*n.
    pub fn from_diatonic_steps(diatonic: i16) -> Self {
        let pitch = diatonic.rem_euclid(7);
        Self(match pitch {
            0 => b'C',
            1 => b'D',
            2 => b'E',
            3 => b'F',
            4 => b'G',
            5 => b'A',
            6 => b'B',
            _ => unreachable!(),
        })
    }

    /// Returns the diatonic steps to C
    pub const fn to_diatonic_steps(&self) -> i16 {
        match self.0 {
            b'C' => 0,
            b'D' => 1,
            b'E' => 2,
            b'F' => 3,
            b'G' => 4,
            b'A' => 5,
            b'B' => 6,
            _ => unreachable!(),
        }
    }

    /// Returns the chromatic steps from C to the natural pitch
    pub fn to_chromatic_steps(&self) -> i16 {
        match self.0 {
            b'C' => 0,
            b'D' => 2,
            b'E' => 4,
            b'F' => 5,
            b'G' => 7,
            b'A' => 9,
            b'B' => 11,
            _ => unreachable!(),
        }
    }

    /// Returns the char for this pitch name
    pub fn as_char(&self) -> char {
        self.0 as char
    }

    /// Returns the pitch name as an ascii byte.
    /// Equivalent to [`.as_char()`][`Self::as_char`] upto the return type.
    pub fn as_ascii_byte(&self) -> u8 {
        self.0
    }
}

/// constants for each pitch name
#[allow(missing_docs)]
impl PitchName {
    pub const C: Self = Self(b'C');
    pub const D: Self = Self(b'D');
    pub const E: Self = Self(b'E');
    pub const F: Self = Self(b'F');
    pub const G: Self = Self(b'G');
    pub const A: Self = Self(b'A');
    pub const B: Self = Self(b'B');
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A type representing pitch
///
/// # Creating Pitch
/// Since the representation of pitch is not intuitive even for someone familiar with music
/// theory, it is suggested to use the implementation of [`FromStr`][`std::str::FromStr`] to create a Pitch.
///
/// To denote the octave scientific pitch notation is used, which defines middle c – the c on the
/// line between bass and treble clef – to be C4 and the start of the 4th octave.
///
/// # Implementation of Ord
/// Pitch implements the [`Ord`][`std::cmp::Ord`] trait.
/// Here the diatonic information is compared before the chromatic information.
/// For any of the usual scales this is irrelevant, since both ways of comparing two pitches will
/// result in the same ordering
/// However, in some edge cases this is relevant:
/// ```
/// # use music_types::harmony::{Pitch, ParsePitchError};
/// # use std::str::FromStr;
/// // as expected
/// assert!(Pitch::from_str("E4")? < Pitch::from_str("F4")?);
///
/// // true because E is one staff space below F
/// assert!(Pitch::from_str("E#4")? < Pitch::from_str("Fb4")?);
///
/// // order flipped because the chromatic pitch of E# is higher than that of Fb
/// assert!(Pitch::from_str("Fb4")?.to_chromatic() < Pitch::from_str("E#4")?.to_chromatic());
/// # Ok::<(), ParsePitchError>(())
/// ```
///
/// One side effect of this implementation of `Ord` is that a sorted list of `Pitch` might not be
/// sorted after converting to ChromaticPitch. If this is undesired use the method `cmp_chromatic`,
/// which compares the chromatic information first.
pub struct Pitch {
    pub(crate) diatonic: i16,
    pub(crate) chromatic: i16,
}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pitch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.diatonic.cmp(&other.diatonic) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.chromatic.cmp(&other.chromatic)
    }
}

impl Pitch {
    /// Compares the pitches by chromatic information first. See struct level docs
    pub fn cmp_chromatic(&self, other: &Self) -> std::cmp::Ordering {
        match self.chromatic.cmp(&other.chromatic) {
            core::cmp::Ordering::Equal => self.diatonic.cmp(&other.diatonic),
            ord => ord,
        }
    }
}

impl Pitch {
    /// Creates a pitch from the chromatic and diatonic steps to middle c (C4)
    ///
    /// # Examples
    /// ```
    /// # use music_types::harmony::{Pitch, ParsePitchError};
    /// # use std::str::FromStr;
    /// assert_eq!(Pitch::new(0, 0), Pitch::from_str("C4")?);
    /// assert_eq!(Pitch::new(1, 2), Pitch::from_str("D4")?);
    /// assert_eq!(Pitch::new(-2, -3), Pitch::from_str("A3")?);
    /// assert_eq!(Pitch::new(7, 12), Pitch::from_str("C5")?);
    /// # Ok::<(), ParsePitchError>(())
    /// ```
    pub fn new(diatonic_steps: i16, chromatic_steps: i16) -> Self {
        Self {
            diatonic: diatonic_steps,
            chromatic: chromatic_steps,
        }
    }

    /// This function decomposes the pitch into its parts in terms of scientific pitch notation
    /// where middle c is the start of the 4th octave.
    pub fn decompose(&self) -> (PitchName, Accidental, i16) {
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

    /// This function is useful when creating a pitch from a pitch class.
    /// alias to `Self::compose(name, accidental, 4)`
    pub fn from_pitch_class(name: PitchName, accidental: Accidental) -> Self {
        Self::compose(name, accidental, 4)
    }

    /// returns the octave of the pitch
    pub fn octave(&self) -> i16 {
        let (octave, _) = div_remainder(self.diatonic, 7);
        octave + 4
    }

    /// return the accidental of the pitch
    pub fn accidental(&self) -> Accidental {
        let (_name, accidental, _octave) = self.decompose();
        accidental
    }

    /// return the name of the pitch
    pub fn pitch_name(&self) -> PitchName {
        let (name, _accidental, _octave) = self.decompose();
        name
    }

    /// this function returns the staff position
    /// middle c is defined to be position 0
    pub fn staff_position(&self) -> i16 {
        self.diatonic
    }

    /// Converts the pitch to a frequency using the standard tuning A4 = 440Hz
    pub fn to_frequency(&self) -> f32 {
        self.to_chromatic().to_frequency()
    }

    /// Converts the pitch to a frequency using the given tuning for A4
    pub fn to_frequency_tuning(&self, a_4: f32) -> f32 {
        self.to_chromatic().to_frequency_tuning(a_4)
    }

    /// Converts to the chromatic pitch
    pub fn to_chromatic(&self) -> ChromaticPitch {
        (*self).into()
    }
}

impl From<Pitch> for ChromaticPitch {
    fn from(value: Pitch) -> Self {
        Self(value.chromatic)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a chromatic pitch
pub struct ChromaticPitch(pub(crate) i16);

impl ChromaticPitch {
    /// construct a chromatic pitch from the number of chromatic steps to middle c
    pub fn new(steps: i16) -> Self {
        Self(steps)
    }

    /// retruns the number of chromatic steps to C4
    pub fn to_num(&self) -> i16 {
        self.0
    }

    /// Converts the pitch to a frequency using the standard tuning A4 = 440Hz
    pub fn to_frequency(&self) -> f32 {
        self.to_frequency_tuning(440.0)
    }

    /// Converts the pitch to a frequency using the given tuning for A4
    pub fn to_frequency_tuning(&self, a_4: f32) -> f32 {
        a_4 * 2.0_f32.powf((self.0 - 9) as f32 / 12.0)
    }

    /// Converts the chromatic pitch to a Pitch
    /// choosing a reasonable diatonic representation.
    pub fn to_pitch(&self) -> Pitch {
        let (_octave, chromatic) = div_remainder(self.0, 12);
        match chromatic {
            0 | 1 => self.to_pitch_named(PitchName(b'C')), // C C#
            2 => self.to_pitch_named(PitchName(b'D')),     // D
            3 | 4 => self.to_pitch_named(PitchName(b'E')), // E Eb
            5 | 6 => self.to_pitch_named(PitchName(b'F')), // F F#
            7 => self.to_pitch_named(PitchName(b'G')),     // G
            8 | 9 => self.to_pitch_named(PitchName(b'A')), // A Ab
            10 | 11 => self.to_pitch_named(PitchName(b'B')), // B Bb
            _ => unreachable!(),
        }
    }

    /// Converts the chromatic pitch to a Pitch so its name is the one given by the name
    pub fn to_pitch_named(&self, name: PitchName) -> Pitch {
        let (mut octave, chromatic) = div_remainder(self.0, 12);
        if chromatic - name.to_chromatic_steps() > 6 {
            octave += 1
        } else if chromatic - name.to_chromatic_steps() > 6 {
            octave -= 1
        }
        Pitch {
            diatonic: octave * 7 + name.to_diatonic_steps(),
            chromatic: self.0,
        }
    }

    const MIDI_C_4: i16 = 60;
    /// Returns the midi pitch if it is within the midi range
    /// The returned u8 is in 0..=127
    pub fn to_midi_pitch(&self) -> Option<u8> {
        // my 0 (middle c) is 60 in MIDI
        let pitch = self.0 + Self::MIDI_C_4;
        if (0..=127).contains(&pitch) {
            Some(pitch as u8)
        } else {
            None
        }
    }

    /// Creates a chromatic pitch from the midi representation
    pub fn from_midi_pitch(pitch: u8) -> Self {
        Self(pitch as i16 - Self::MIDI_C_4)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    #[test]
    fn convert() {
        let pitch = ChromaticPitch::new(0);
        assert_eq!(Pitch::from_str("C4").unwrap(), pitch.to_pitch());
        assert_eq!(
            Pitch::from_str("C4").unwrap(),
            pitch.to_pitch_named(PitchName(b'C'))
        );
        assert_eq!(
            Pitch::from_str("D&4").unwrap(),
            pitch.to_pitch_named(PitchName(b'D'))
        );

        let pitch = ChromaticPitch::new(1);
        assert_eq!(Pitch::from_str("C#4").unwrap(), pitch.to_pitch());
        assert_eq!(
            Pitch::from_str("Db4").unwrap(),
            pitch.to_pitch_named(PitchName(b'D'))
        );

        let pitch = ChromaticPitch::new(-1);
        assert_eq!(Pitch::from_str("B3").unwrap(), pitch.to_pitch());
        assert_eq!(
            Pitch::from_str("Cb4").unwrap(),
            pitch.to_pitch_named(PitchName(b'C'))
        );
    }
}
