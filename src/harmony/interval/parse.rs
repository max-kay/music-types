use std::{error::Error, fmt, str::FromStr};

use super::Interval;

#[derive(Debug)]
/// Error that may occur when parsing an interval.
pub enum ParseIntervalError {
    /// Error from an invalid interval number
    InvalidNumber(String),
    /// Error from an invalid interval quality
    InvalidQuality(String),
    /// Error from a impossible combination of quality and degree
    Impossible {
        /// the number of the interval
        number: i16,
        /// the string from which the quality was tried to parse
        /// None means the string was empty ie perfect chord quality
        quality: Option<String>,
    },
}

impl fmt::Display for ParseIntervalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseIntervalError::InvalidNumber(s) => {
                write!(f, "could not parse interval number `{s}`")
            }
            ParseIntervalError::InvalidQuality(s) => {
                write!(f, "could not parse interval quality `{s}`")
            }
            ParseIntervalError::Impossible {
                number,
                quality: Some(quality),
            } => {
                write!(
                    f,
                    "interval of number {number} (octave equivalent to {}) cannot have quality `{quality}`",
                    (number.abs() - 1) % 7 + 1
                )
            }
            ParseIntervalError::Impossible {
                number,
                quality: None,
            } => {
                write!(
                    f,
                    "interval of number {number} (octave equivalent to {}) cannot be perfect",
                    (number.abs() - 1) % 7 + 1
                )
            }
        }
    }
}

impl Error for ParseIntervalError {}

impl FromStr for Interval {
    type Err = ParseIntervalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(sub_string) = s.strip_prefix('-') {
            return Self::from_str(sub_string).map(|i| -i);
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
            return Err(ParseIntervalError::InvalidNumber(String::new()));
        }
        let digits: String = digits.chars().rev().collect();
        let interval_number = i16::from_str(&digits).expect("can only contain digits");
        let diatonic_steps = interval_number - 1;
        let chromatic_steps: i16 = match chars[..chars.len() - digits.len()] {
            [] => {
                if Self::has_perfect(diatonic_steps) {
                    Self::to_chromatic_steps_perfect(diatonic_steps)
                } else {
                    return Err(ParseIntervalError::Impossible {
                        number: interval_number,
                        quality: None,
                    });
                }
            }
            [c] => {
                if Self::has_perfect(diatonic_steps) {
                    let perfect_steps = Self::to_chromatic_steps_perfect(diatonic_steps);
                    match c {
                        'a' | 'A' => perfect_steps + 1,
                        'j' | 'M' => {
                            return Err(ParseIntervalError::Impossible {
                                number: interval_number,
                                quality: Some(c.to_string()),
                            });
                        }
                        'p' | 'P' => perfect_steps,
                        'm' => {
                            return Err(ParseIntervalError::Impossible {
                                number: interval_number,
                                quality: Some(c.to_string()),
                            });
                        }
                        'd' => perfect_steps - 1,
                        _ => return Err(ParseIntervalError::InvalidQuality(c.into())),
                    }
                } else {
                    let minor_steps = Self::to_chromatic_steps_minor(diatonic_steps);
                    match c {
                        'a' | 'A' => minor_steps + 2,
                        'j' | 'M' => minor_steps + 1,
                        'p' | 'P' => {
                            return Err(ParseIntervalError::Impossible {
                                number: interval_number,
                                quality: Some(c.to_string()),
                            });
                        }
                        'm' => minor_steps,
                        'd' => minor_steps - 1,
                        _ => return Err(ParseIntervalError::InvalidQuality(c.into())),
                    }
                }
            }
            ['(', '+', ref middle @ .., ')'] | ['(', ref middle @ .., ')'] => {
                let as_string: String = middle.iter().collect();
                let quality: i16 = if let Ok(num) = FromStr::from_str(&as_string) {
                    num
                } else {
                    return Err(ParseIntervalError::InvalidQuality(as_string));
                };
                if Self::has_perfect(diatonic_steps) {
                    let nat_steps = Self::to_chromatic_steps_perfect(diatonic_steps);
                    match quality {
                        (i16::MIN..=-2) => nat_steps + quality + 1,
                        0 => nat_steps,
                        (2..=i16::MAX) => nat_steps + quality - 1,
                        -1 | 1 => {
                            return Err(ParseIntervalError::Impossible {
                                number: interval_number,
                                quality: Some(as_string),
                            });
                        }
                    }
                } else {
                    let minor_steps = Self::to_chromatic_steps_minor(diatonic_steps);
                    match quality {
                        (i16::MIN..=-1) => minor_steps + quality + 1,
                        (1..=i16::MAX) => minor_steps + quality,
                        0 => {
                            return Err(ParseIntervalError::Impossible {
                                number: interval_number,
                                quality: Some(as_string),
                            });
                        }
                    }
                }
            }
            _ => return Err(ParseIntervalError::InvalidQuality(chars.iter().collect())),
        };

        Ok(Self {
            chromatic: chromatic_steps,
            diatonic: diatonic_steps,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

        parse_i!("(3)1", 0, 2);
        parse_i!("(2)1", 0, 1);
        parse_i!("(0)1", 0, 0);
        parse_i!("(-2)1", 0, -1);
        parse_i!("(-3)1", 0, -2);

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
