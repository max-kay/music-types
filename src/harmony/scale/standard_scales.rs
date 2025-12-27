#![allow(missing_docs)]
use super::*;

/// Common scales
impl Scale {
    pub fn major() -> Self {
        Self(vec![
            Interval {
                chromatic: 0,
                diatonic: 0,
            },
            Interval {
                chromatic: 2,
                diatonic: 1,
            },
            Interval {
                chromatic: 4,
                diatonic: 2,
            },
            Interval {
                chromatic: 5,
                diatonic: 3,
            },
            Interval {
                chromatic: 7,
                diatonic: 4,
            },
            Interval {
                chromatic: 9,
                diatonic: 5,
            },
            Interval {
                chromatic: 11,
                diatonic: 6,
            },
        ])
    }

    pub fn minor() -> Self {
        Self(vec![
            Interval {
                chromatic: 0,
                diatonic: 0,
            },
            Interval {
                chromatic: 2,
                diatonic: 1,
            },
            Interval {
                chromatic: 3,
                diatonic: 2,
            },
            Interval {
                chromatic: 5,
                diatonic: 3,
            },
            Interval {
                chromatic: 7,
                diatonic: 4,
            },
            Interval {
                chromatic: 8,
                diatonic: 5,
            },
            Interval {
                chromatic: 10,
                diatonic: 6,
            },
        ])
    }

    pub fn harmonic_minor() -> Self {
        Self(vec![
            Interval {
                chromatic: 0,
                diatonic: 0,
            },
            Interval {
                chromatic: 2,
                diatonic: 1,
            },
            Interval {
                chromatic: 3,
                diatonic: 2,
            },
            Interval {
                chromatic: 5,
                diatonic: 3,
            },
            Interval {
                chromatic: 7,
                diatonic: 4,
            },
            Interval {
                chromatic: 8,
                diatonic: 5,
            },
            Interval {
                chromatic: 11,
                diatonic: 6,
            },
        ])
    }

    pub fn melodic_minor() -> Self {
        Self(vec![
            Interval {
                chromatic: 0,
                diatonic: 0,
            },
            Interval {
                chromatic: 2,
                diatonic: 1,
            },
            Interval {
                chromatic: 3,
                diatonic: 2,
            },
            Interval {
                chromatic: 5,
                diatonic: 3,
            },
            Interval {
                chromatic: 7,
                diatonic: 4,
            },
            Interval {
                chromatic: 9,
                diatonic: 5,
            },
            Interval {
                chromatic: 11,
                diatonic: 6,
            },
        ])
    }

    pub fn lydian() -> Self {
        Self(vec![
            Interval {
                chromatic: 0,
                diatonic: 0,
            },
            Interval {
                chromatic: 2,
                diatonic: 1,
            },
            Interval {
                chromatic: 4,
                diatonic: 2,
            },
            Interval {
                chromatic: 6,
                diatonic: 3,
            },
            Interval {
                chromatic: 7,
                diatonic: 4,
            },
            Interval {
                chromatic: 9,
                diatonic: 5,
            },
            Interval {
                chromatic: 11,
                diatonic: 6,
            },
        ])
    }

    pub fn ionian() -> Self {
        Self::major()
    }

    pub fn mixolydian() -> Self {
        Self(vec![
            Interval {
                chromatic: 0,
                diatonic: 0,
            },
            Interval {
                chromatic: 2,
                diatonic: 1,
            },
            Interval {
                chromatic: 4,
                diatonic: 2,
            },
            Interval {
                chromatic: 5,
                diatonic: 3,
            },
            Interval {
                chromatic: 7,
                diatonic: 4,
            },
            Interval {
                chromatic: 9,
                diatonic: 5,
            },
            Interval {
                chromatic: 10,
                diatonic: 6,
            },
        ])
    }

    pub fn dorian() -> Self {
        Self(vec![
            Interval {
                chromatic: 0,
                diatonic: 0,
            },
            Interval {
                chromatic: 2,
                diatonic: 1,
            },
            Interval {
                chromatic: 3,
                diatonic: 2,
            },
            Interval {
                chromatic: 5,
                diatonic: 3,
            },
            Interval {
                chromatic: 7,
                diatonic: 4,
            },
            Interval {
                chromatic: 9,
                diatonic: 5,
            },
            Interval {
                chromatic: 10,
                diatonic: 6,
            },
        ])
    }

    pub fn aeolian() -> Self {
        Self::minor()
    }

    pub fn phrygian() -> Self {
        Self(vec![
            Interval {
                chromatic: 0,
                diatonic: 0,
            },
            Interval {
                chromatic: 1,
                diatonic: 1,
            },
            Interval {
                chromatic: 3,
                diatonic: 2,
            },
            Interval {
                chromatic: 5,
                diatonic: 3,
            },
            Interval {
                chromatic: 7,
                diatonic: 4,
            },
            Interval {
                chromatic: 8,
                diatonic: 5,
            },
            Interval {
                chromatic: 10,
                diatonic: 6,
            },
        ])
    }

    pub fn locrian() -> Self {
        Self(vec![
            Interval {
                chromatic: 0,
                diatonic: 0,
            },
            Interval {
                chromatic: 1,
                diatonic: 1,
            },
            Interval {
                chromatic: 3,
                diatonic: 2,
            },
            Interval {
                chromatic: 5,
                diatonic: 3,
            },
            Interval {
                chromatic: 6,
                diatonic: 4,
            },
            Interval {
                chromatic: 8,
                diatonic: 5,
            },
            Interval {
                chromatic: 10,
                diatonic: 6,
            },
        ])
    }
}

/// constants for the most common Keysignatures
#[allow(missing_docs)]
impl KeyAccidental {
    pub const C_FLAT: Self = Self {
        staffposition: 0,
        accidental: Accidental::new(-1),
    };
    pub const G_FLAT: Self = Self {
        staffposition: 4,
        accidental: Accidental::new(-1),
    };
    pub const D_FLAT: Self = Self {
        staffposition: 1,
        accidental: Accidental::new(-1),
    };
    pub const A_FLAT: Self = Self {
        staffposition: 5,
        accidental: Accidental::new(-1),
    };
    pub const E_FLAT: Self = Self {
        staffposition: 2,
        accidental: Accidental::new(-1),
    };
    pub const B_FLAT: Self = Self {
        staffposition: 6,
        accidental: Accidental::new(-1),
    };

    pub const F_SHARP: Self = Self {
        staffposition: 3,
        accidental: Accidental::new(1),
    };
    pub const C_SHARP: Self = Self {
        staffposition: 0,
        accidental: Accidental::new(1),
    };
    pub const G_SHARP: Self = Self {
        staffposition: 4,
        accidental: Accidental::new(1),
    };
    pub const D_SHARP: Self = Self {
        staffposition: 1,
        accidental: Accidental::new(1),
    };
    pub const A_SHARP: Self = Self {
        staffposition: 5,
        accidental: Accidental::new(1),
    };
    pub const E_SHARP: Self = Self {
        staffposition: 2,
        accidental: Accidental::new(1),
    };
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
