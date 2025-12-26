use std::{error::Error, fmt, str::FromStr};

use super::{Accidental, Pitch, PitchName};

#[derive(Debug)]
/// Error that may occur when parsing a pitch.
pub enum ParsePitchError {
    /// An invalid pitch name –
    /// see [`PitchName`]
    InvalidPitchName(String),
    /// An invalid accidental –
    /// see [`Accidental`]
    InvalidAccidental(String),
    /// No octave was found.
    NoOctaveFound,
    /// An invalid octave was found.
    InvalidOctave(String),
}

impl fmt::Display for ParsePitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsePitchError::InvalidPitchName(s) => write!(f, "pitch name `{s}` is invalid"),
            ParsePitchError::InvalidAccidental(s) => write!(f, "accidental `{s}` is invalid"),
            ParsePitchError::NoOctaveFound => write!(f, "no octave in string"),
            ParsePitchError::InvalidOctave(s) => write!(f, "could not parse octave `{s}`"),
        }
    }
}

impl Error for ParsePitchError {}

impl FromStr for Accidental {
    type Err = ParsePitchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "##" | "+" | "\u{1d12b}" => return Ok(Self(2)),
            "#" | "\u{266f}" => return Ok(Self(1)),
            "n" | "" | "\u{266e}" => return Ok(Self(0)),
            "b" | "\u{266d}" => return Ok(Self(-1)),
            "bb" | "&" | "\u{1d12a}" => return Ok(Self(-2)),
            _ => (),
        }

        if let Some(s) = s.strip_prefix("(") {
            if let Some(s) = s.strip_suffix(")") {
                let num: u8 = FromStr::from_str(&s[0..s.len() - 1])
                    .map_err(|_| ParsePitchError::InvalidAccidental(s.to_string()))?;
                match s.chars().last() {
                    Some('#') => return Ok(Self(num as i16)),
                    Some('b') => return Ok(Self(-(num as i16))),
                    _ => return Err(ParsePitchError::InvalidAccidental(s.to_string())),
                }
            } else {
                return Err(ParsePitchError::InvalidAccidental(s.to_string()));
            }
        }
        let mut char_iter = s.chars();
        let first = char_iter
            .next()
            .expect("empty string is already matched as natural");
        if !char_iter.all(|c| c == first) {
            return Err(ParsePitchError::InvalidAccidental(s.to_string()));
        }
        match first {
            '#' => Ok(Self(s.len() as i16)),
            'b' => Ok(Self(-(s.len() as i16))),
            _ => return Err(ParsePitchError::InvalidAccidental(s.to_string())),
        }
    }
}

impl FromStr for PitchName {
    type Err = ParsePitchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars();
        match (c.next(), c.next()) {
            (Some(c), None) if ('A'..='G').contains(&c) => Ok(Self(c as u8)),
            _ => Err(ParsePitchError::InvalidPitchName(s.to_string())),
        }
    }
}

impl FromStr for Pitch {
    type Err = ParsePitchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octave_index = s
            .char_indices()
            .rev()
            .find(|&(_, c)| !c.is_ascii_digit())
            .map(|(i, _)| i)
            .ok_or(ParsePitchError::NoOctaveFound)?
            + 1;
        if s.chars()
            .nth(octave_index - 1)
            .ok_or(ParsePitchError::InvalidPitchName(s.to_string()))?
            == '-'
        {
            octave_index -= 1;
        }
        let octave_str = &s[octave_index..];
        let octave: i16 = FromStr::from_str(octave_str)
            .map_err(|_| ParsePitchError::InvalidOctave(octave_str.to_string()))?;
        let mut chars = s[0..octave_index].chars();
        let pitch_name = PitchName::new(chars.next().ok_or(ParsePitchError::InvalidPitchName(
            s[0..octave_index].to_string(),
        ))?)
        .ok_or(ParsePitchError::InvalidPitchName(
            s[0..octave_index].to_string(),
        ))?;
        Ok(Self::compose(
            pitch_name,
            Accidental::from_str(chars.as_str())?,
            octave,
        ))
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
}
