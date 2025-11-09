#![allow(missing_docs)]
use super::*;

/// Diatonic modes
impl Scale {
    pub fn major() -> Self {
        Self(vec![
            IntervalClass(Interval {
                chromatic: 0,
                diatonic: 0,
            }),
            IntervalClass(Interval {
                chromatic: 2,
                diatonic: 1,
            }),
            IntervalClass(Interval {
                chromatic: 4,
                diatonic: 2,
            }),
            IntervalClass(Interval {
                chromatic: 5,
                diatonic: 3,
            }),
            IntervalClass(Interval {
                chromatic: 7,
                diatonic: 4,
            }),
            IntervalClass(Interval {
                chromatic: 9,
                diatonic: 5,
            }),
            IntervalClass(Interval {
                chromatic: 11,
                diatonic: 6,
            }),
        ])
    }

    pub fn minor() -> Self {
        Self(vec![
            IntervalClass(Interval {
                chromatic: 0,
                diatonic: 0,
            }),
            IntervalClass(Interval {
                chromatic: 2,
                diatonic: 1,
            }),
            IntervalClass(Interval {
                chromatic: 3,
                diatonic: 2,
            }),
            IntervalClass(Interval {
                chromatic: 5,
                diatonic: 3,
            }),
            IntervalClass(Interval {
                chromatic: 7,
                diatonic: 4,
            }),
            IntervalClass(Interval {
                chromatic: 8,
                diatonic: 5,
            }),
            IntervalClass(Interval {
                chromatic: 10,
                diatonic: 6,
            }),
        ])
    }

    pub fn lydian() -> Self {
        Self(vec![
            IntervalClass(Interval {
                chromatic: 0,
                diatonic: 0,
            }),
            IntervalClass(Interval {
                chromatic: 2,
                diatonic: 1,
            }),
            IntervalClass(Interval {
                chromatic: 4,
                diatonic: 2,
            }),
            IntervalClass(Interval {
                chromatic: 6,
                diatonic: 3,
            }),
            IntervalClass(Interval {
                chromatic: 7,
                diatonic: 4,
            }),
            IntervalClass(Interval {
                chromatic: 9,
                diatonic: 5,
            }),
            IntervalClass(Interval {
                chromatic: 11,
                diatonic: 6,
            }),
        ])
    }

    pub fn ionian() -> Self {
        Self::major()
    }

    pub fn mixolydian() -> Self {
        Self(vec![
            IntervalClass(Interval {
                chromatic: 0,
                diatonic: 0,
            }),
            IntervalClass(Interval {
                chromatic: 2,
                diatonic: 1,
            }),
            IntervalClass(Interval {
                chromatic: 4,
                diatonic: 2,
            }),
            IntervalClass(Interval {
                chromatic: 5,
                diatonic: 3,
            }),
            IntervalClass(Interval {
                chromatic: 7,
                diatonic: 4,
            }),
            IntervalClass(Interval {
                chromatic: 9,
                diatonic: 5,
            }),
            IntervalClass(Interval {
                chromatic: 10,
                diatonic: 6,
            }),
        ])
    }

    pub fn dorian() -> Self {
        Self(vec![
            IntervalClass(Interval {
                chromatic: 0,
                diatonic: 0,
            }),
            IntervalClass(Interval {
                chromatic: 2,
                diatonic: 1,
            }),
            IntervalClass(Interval {
                chromatic: 3,
                diatonic: 2,
            }),
            IntervalClass(Interval {
                chromatic: 5,
                diatonic: 3,
            }),
            IntervalClass(Interval {
                chromatic: 7,
                diatonic: 4,
            }),
            IntervalClass(Interval {
                chromatic: 9,
                diatonic: 5,
            }),
            IntervalClass(Interval {
                chromatic: 10,
                diatonic: 6,
            }),
        ])
    }

    pub fn aeolian() -> Self {
        Self::minor()
    }

    pub fn phrygian() -> Self {
        Self(vec![
            IntervalClass(Interval {
                chromatic: 0,
                diatonic: 0,
            }),
            IntervalClass(Interval {
                chromatic: 1,
                diatonic: 1,
            }),
            IntervalClass(Interval {
                chromatic: 3,
                diatonic: 2,
            }),
            IntervalClass(Interval {
                chromatic: 5,
                diatonic: 3,
            }),
            IntervalClass(Interval {
                chromatic: 7,
                diatonic: 4,
            }),
            IntervalClass(Interval {
                chromatic: 8,
                diatonic: 5,
            }),
            IntervalClass(Interval {
                chromatic: 10,
                diatonic: 6,
            }),
        ])
    }

    pub fn test_thing() -> Self {
        std::str::FromStr::from_str("1 2 3 4 5 6 7").unwrap()
    }

    pub fn locrian() -> Self {
        Self(vec![
            IntervalClass(Interval {
                chromatic: 0,
                diatonic: 0,
            }),
            IntervalClass(Interval {
                chromatic: 1,
                diatonic: 1,
            }),
            IntervalClass(Interval {
                chromatic: 3,
                diatonic: 2,
            }),
            IntervalClass(Interval {
                chromatic: 5,
                diatonic: 3,
            }),
            IntervalClass(Interval {
                chromatic: 6,
                diatonic: 4,
            }),
            IntervalClass(Interval {
                chromatic: 8,
                diatonic: 5,
            }),
            IntervalClass(Interval {
                chromatic: 10,
                diatonic: 6,
            }),
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn diatonic_modes() {
        let major = Scale::major();
        assert_eq!(major.nth_mode(1), Scale::dorian());
        assert_eq!(major.nth_mode(2), Scale::phrygian());
        assert_eq!(major.nth_mode(3), Scale::lydian());
        assert_eq!(major.nth_mode(4), Scale::mixolydian());
        assert_eq!(major.nth_mode(5), Scale::aeolian());
        assert_eq!(major.nth_mode(6), Scale::locrian());
    }
}
