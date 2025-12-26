use std::fmt::{self, Display};

use super::Interval;

impl Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.diatonic < 0 {
            return write!(f, "-{}", -self);
        }
        let modifier = if Self::has_perfect(self.diatonic) {
            let mismatch = self.chromatic - Self::to_chromatic_steps_perfect(self.diatonic);
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
            let mismatch = self.chromatic - Self::to_chromatic_steps_minor(self.diatonic);
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
