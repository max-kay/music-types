use super::*;
use std::{error::Error, fmt, i16, ops::Neg, str::FromStr};

#[derive(Debug)]
/// Represents an error from parsing.
pub enum PitchParseError {
    /// An invalid pitch name
    InvalidPitchName(String),
    /// An accidental
    InvalidAccidental(String),
    /// no ocatave found
    NoOctaveFound,
    /// invalid octave
    InvalidOctave(String),
}

impl fmt::Display for PitchParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for PitchParseError {}

impl FromStr for PitchName {
    type Err = PitchParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars();
        match (c.next(), c.next()) {
            (Some(c), None) if ('A'..='G').contains(&c) => Ok(Self(c as u8)),
            _ => Err(PitchParseError::InvalidPitchName(s.to_string())),
        }
    }
}

impl FromStr for Pitch {
    type Err = PitchParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octave_index = s
            .char_indices()
            .rev()
            .find(|&(_, c)| !c.is_ascii_digit())
            .map(|(i, _)| i)
            .ok_or(PitchParseError::NoOctaveFound)?
            + 1;
        if s.chars()
            .nth(octave_index - 1)
            .ok_or(PitchParseError::InvalidPitchName(s.to_string()))?
            == '-'
        {
            octave_index -= 1;
        }
        let octave_str = &s[octave_index..];
        let pitch = PitchClass::from_str(&s[0..octave_index])?;
        let octave: i8 = FromStr::from_str(octave_str)
            .map_err(|_| PitchParseError::InvalidOctave(octave_str.to_string()))?;
        Ok(pitch.place_in_octave(octave))
    }
}

impl FromStr for PitchClass {
    type Err = PitchParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let pitch_name = PitchName::new(
            chars
                .next()
                .ok_or(PitchParseError::InvalidPitchName(s.to_string()))?,
        )
        .ok_or(PitchParseError::InvalidPitchName(s.to_string()))?;
        Ok(Self::compose(
            pitch_name,
            Accidental::from_str(chars.as_str())?,
        ))
    }
}

impl FromStr for Accidental {
    type Err = PitchParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(match s {
            "##" | "+" => 2,
            "#" => 1,
            "n" | "" => 0,
            "b" => -1,
            "bb" | "&" => -2,
            s => {
                if let Some(s) = s.strip_prefix("(") {
                    if let Some(s) = s.strip_suffix(")") {
                        let num: u8 = FromStr::from_str(&s[0..s.len() - 1])
                            .map_err(|_| PitchParseError::InvalidAccidental(s.to_string()))?;
                        match s.chars().last() {
                            Some('#') => return Ok(Self(num as i16)),
                            Some('b') => return Ok(Self(-(num as i16))),
                            _ => return Err(PitchParseError::InvalidAccidental(s.to_string())),
                        }
                    } else {
                        return Err(PitchParseError::InvalidAccidental(s.to_string()));
                    }
                }
                let mut char_iter = s.chars();
                let first = char_iter
                    .next()
                    .expect("empty string is already matched as natural");
                if !char_iter.all(|c| c == first) {
                    return Err(PitchParseError::InvalidAccidental(s.to_string()));
                }
                match first {
                    '#' => s.len() as i16,
                    'b' => -(s.len() as i16),
                    _ => return Err(PitchParseError::InvalidAccidental(s.to_string())),
                }
            }
        }))
    }
}

#[derive(Debug)]
pub enum IntervalParseError {
    InvalidInterval(String),
    InvalidNumber(String),
    InvalidModifier(String),
    Impossible,
}

impl FromStr for Interval {
    type Err = IntervalParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(sub_string) = s.strip_prefix('-') {
            return Self::from_str(sub_string).map(Neg::neg);
        }
        let chars: Vec<char> = s.chars().collect();
        let mut digits = String::new();
        for c in chars.iter().rev() {
            if !c.is_ascii_digit() {
                break;
            }
            digits.push(*c);
        }
        if digits.is_empty() {
            return Err(IntervalParseError::InvalidNumber(String::new()));
        }
        let digits: String = digits.chars().rev().collect();
        let diatonic_steps = i16::from_str(&digits).expect("can only contain digits") - 1;
        let modifier: i16 = match chars[..chars.len() - digits.len()] {
            [] => 0,
            [c] => match c {
                'a' => 2,
                'j' => 1,
                'p' => 0,
                'm' => -1,
                'd' => -2,
                _ => return Err(IntervalParseError::InvalidModifier(c.into())),
            },
            ['(', '+', ref middle @ .., ')'] | ['(', ref middle @ .., ')'] => {
                let as_string: String = middle.iter().collect();
                FromStr::from_str(&as_string)
                    .map_err(|_| IntervalParseError::InvalidModifier(as_string))?
            }
            _ => return Err(IntervalParseError::InvalidModifier(chars.iter().collect())),
        };

        let chromatic_steps = if Self::has_perfect(diatonic_steps) {
            let nat_steps = Self::to_diatonic_steps_perfect(diatonic_steps);
            match modifier {
                (i16::MIN..=-2) => nat_steps + modifier + 1,
                0 => nat_steps,
                (2..=i16::MAX) => nat_steps + modifier - 1,
                -1 | 1 => return Err(IntervalParseError::Impossible),
            }
        } else {
            let minor_steps = Self::to_diatonic_steps_minor(diatonic_steps);
            match modifier {
                (i16::MIN..=-1) => minor_steps + modifier + 1,
                (1..=i16::MAX) => minor_steps + modifier,
                0 => return Err(IntervalParseError::Impossible),
            }
        };

        Ok(Self {
            chromatic: chromatic_steps,
            diatonic: diatonic_steps,
        })
    }
}

impl FromStr for IntervalClass {
    type Err = IntervalParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(FromStr::from_str(s)?).reduce())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[rustfmt::skip]
    macro_rules! parse_p {
        ($s:literal, $d:literal, $c:literal) => {
            let p = Pitch::from_str($s).unwrap();
            assert_eq!(p.diatonic, $d, "{} parsed to {} diatonic steps", $s, p.diatonic);
            assert_eq!(p.chromatic, $c, "{} parsed to {} chromatic steps", $s, p.chromatic);
        };
    }

    #[test]
    fn parse_pitch() {
        parse_p!("Cb4", 0, -1);
        parse_p!("C4", 0, 0);
        parse_p!("C#4", 0, 1);
        parse_p!("Db4", 1, 1);
        parse_p!("D4", 1, 2);
        parse_p!("D#4", 1, 3);
        parse_p!("Eb4", 2, 3);
        parse_p!("E4", 2, 4);
        parse_p!("E#4", 2, 5);
        parse_p!("Fb4", 3, 4);
        parse_p!("F4", 3, 5);
        parse_p!("F#4", 3, 6);
        parse_p!("Gb4", 4, 6);
        parse_p!("G4", 4, 7);
        parse_p!("G#4", 4, 8);
        parse_p!("Ab4", 5, 8);
        parse_p!("A4", 5, 9);
        parse_p!("A#4", 5, 10);
        parse_p!("Bb4", 6, 10);
        parse_p!("B4", 6, 11);
        parse_p!("B#4", 6, 12);

        parse_p!("C5", 7, 12);
        parse_p!("D5", 8, 14);
        parse_p!("C3", -7, -12);
        parse_p!("Bb2", -8, -14);

        parse_p!("C+4", 0, 2);
        parse_p!("C##4", 0, 2);
        parse_p!("C&4", 0, -2);
        parse_p!("Cbb4", 0, -2);

        parse_p!("C###4", 0, 3);
        parse_p!("C(3#)4", 0, 3);
        parse_p!("Cbbb4", 0, -3);
        parse_p!("C(3b)4", 0, -3);
    }

    #[test]
    fn parse_pitch_fail() {
        assert!(Pitch::from_str("C").is_err());
        assert!(Pitch::from_str("Ch").is_err());
        assert!(Pitch::from_str("c18").is_err());
    }

    macro_rules! parse_i {
        ($s:literal, $d:literal, $c:literal) => {
            let i = Interval::from_str($s).unwrap();
            assert_eq!(
                i.diatonic, $d,
                "{} parsed to {} diatonic steps",
                $s, i.diatonic
            );
            assert_eq!(
                i.chromatic, $c,
                "{} parsed to {} chromatic steps",
                $s, i.chromatic
            );
        };
    }

    #[test]
    fn parse_interval() {
        parse_i!("1", 0, 0);
        parse_i!("m2", 1, 1);
        parse_i!("j2", 1, 2);
        parse_i!("m3", 2, 3);
        parse_i!("j3", 2, 4);
        parse_i!("4", 3, 5);
        parse_i!("a4", 3, 6);
        parse_i!("d5", 4, 6);
        parse_i!("5", 4, 7);
        parse_i!("m6", 5, 8);
        parse_i!("j6", 5, 9);
        parse_i!("m7", 6, 10);
        parse_i!("j7", 6, 11);
        parse_i!("8", 7, 12);

        parse_i!("d3", 2, 2);
        parse_i!("a3", 2, 5);
        parse_i!("-j2", -1, -2);
        parse_i!("-15", -14, -24);
        parse_i!("a11", 10, 18);

        parse_i!("(2)1", 0, 1);
        parse_i!("(0)1", 0, 0);
        parse_i!("(-2)1", 0, -1);

        parse_i!("(+3)3", 2, 6);
        parse_i!("(3)3", 2, 6);
        parse_i!("(+2)3", 2, 5);
        parse_i!("(2)3", 2, 5);
        parse_i!("(+1)3", 2, 4);
        parse_i!("(1)3", 2, 4);
        parse_i!("(-1)3", 2, 3);
        parse_i!("(-2)3", 2, 2);
        parse_i!("(-3)3", 2, 1);
        parse_i!("(-4)3", 2, 0);
        parse_i!("(-5)3", 2, -1);
    }

    #[test]
    fn parse_interval_fail() {
        assert!(Interval::from_str("m1").is_err());
        assert!(Interval::from_str("3").is_err());
        assert!(Interval::from_str("16").is_err());
        assert!(Interval::from_str("m11").is_err());
        assert!(Interval::from_str("m15").is_err());
        assert!(Interval::from_str("(0)3").is_err());
        assert!(Interval::from_str("(-1)1").is_err());
        assert!(Interval::from_str("(+1)1").is_err());
        assert!(Interval::from_str("(1)1").is_err());
    }
}
