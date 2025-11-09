use std::{
    fmt::{self, Display},
    i16,
};

use super::*;

impl Display for PitchName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl fmt::Debug for PitchName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PitchName({})", self)
    }
}

impl Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (c, acc) = self.decompose();
        write!(f, "{}{}", c, acc)
    }
}

impl fmt::Debug for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PitchClass({})", self)
    }
}

impl Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (c, acc, oct) = self.decompose();
        write!(f, "{}{}{}", c, acc, oct)
    }
}

impl fmt::Debug for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pitch({})", self)
    }
}

impl Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.0 {
            0 => "",
            1 => "#",
            -1 => "b",
            2 => "+",
            -2 => "&",
            n if n > 0 => &("#".repeat(n as usize)),
            n => &("b".repeat(n.abs() as usize)),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Debug for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Accidental({})", self)
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.diatonic < 0 {
            return write!(f, "-{}", -self);
        }
        let modifier = if Self::has_perfect(self.diatonic) {
            let mismatch = self.chromatic - Self::to_diatonic_steps_perfect(self.diatonic);
            match mismatch {
                // -1 so that in string form -2 can represent dimished intervals
                (i16::MIN..=-2) => &format!("({})", mismatch - 1),
                -1 => "d",
                0 => "",
                1 => "a",
                // +1 so that in string form 2 can represent augmented intervals
                (2..=i16::MAX) => &format!("({})", mismatch + 1),
            }
        } else {
            let mismatch = self.chromatic - Self::to_diatonic_steps_minor(self.diatonic);
            match mismatch {
                // -1 so that in string form -2 can represent dimished intervals
                (i16::MIN..=-2) => &format!("({})", mismatch - 1),
                -1 => "d",
                0 => "m",
                1 => "j",
                2 => "a",
                (3..=i16::MAX) => &format!("({})", mismatch),
            }
        };
        write!(f, "{}{}", modifier, self.diatonic + 1)
    }
}

impl fmt::Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Interval({})", self)
    }
}

impl Display for IntervalClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Debug for IntervalClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntervalClass({})", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    macro_rules! display {
        ($t:ty, $i:literal) => {
            display!($t, $i, $i);
        };
        ($t:ty, $i1:literal, $i2:literal) => {
            assert_eq!(&format!("{}", <$t>::from_str($i1).unwrap()), $i2)
        };
    }

    #[test]
    fn pitch() {
        display!(Pitch, "Eb4");
        display!(Pitch, "Ebb4", "E&4");
        display!(Pitch, "E4");
        display!(Pitch, "F5");
    }

    #[test]
    fn pitch_class() {
        display!(PitchClass, "Cb");
        display!(PitchClass, "C");
        display!(PitchClass, "C#");
        display!(PitchClass, "Db");
        display!(PitchClass, "D");
        display!(PitchClass, "D#");
        display!(PitchClass, "Eb");
        display!(PitchClass, "E");
        display!(PitchClass, "E#");
        display!(PitchClass, "Fb");
        display!(PitchClass, "F");
        display!(PitchClass, "F#");
        display!(PitchClass, "Gb");
        display!(PitchClass, "G");
        display!(PitchClass, "G#");
        display!(PitchClass, "Ab");
        display!(PitchClass, "A");
        display!(PitchClass, "A#");
        display!(PitchClass, "Bb");
        display!(PitchClass, "B");
        display!(PitchClass, "B#");
    }

    #[test]
    fn interval() {
        display!(Interval, "1");
        display!(Interval, "4");
        display!(Interval, "5");
        display!(Interval, "8");

        display!(Interval, "j3");
        display!(Interval, "-j3");

        display!(Interval, "a1");
        display!(Interval, "a4");
        display!(Interval, "a5");
        display!(Interval, "a8");

        display!(Interval, "d1");
        display!(Interval, "d4");
        display!(Interval, "d5");
        display!(Interval, "d8");
    }
}
