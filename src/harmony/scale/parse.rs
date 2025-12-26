use std::str::FromStr;

use crate::harmony::{Interval, ParseError};

use super::Scale;

impl FromStr for Scale {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_whitespace()
                .map(|s| match s {
                    "2" => Ok(Interval {
                        chromatic: 2,
                        diatonic: 1,
                    }),
                    "3" => Ok(Interval {
                        chromatic: 4,
                        diatonic: 2,
                    }),
                    "6" => Ok(Interval {
                        chromatic: 9,
                        diatonic: 5,
                    }),
                    "7" => Ok(Interval {
                        chromatic: 10,
                        diatonic: 6,
                    }),
                    s => Interval::from_str(s).map_err(|e| ParseError::Interval(e)),
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse() {
        assert_eq!(
            Scale::from_str("1 2 3 4 5 6 7").unwrap(),
            Scale::mixolydian()
        );
        assert_eq!(
            Scale::from_str("1 j2 j3 4 5 j6 m7").unwrap(),
            Scale::mixolydian()
        );
        assert_eq!(Scale::from_str("1 2 3 4 5 6 j7").unwrap(), Scale::major());
    }
}
