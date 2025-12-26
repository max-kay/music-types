use std::fmt::{self, Display};

use super::*;

impl Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.0 {
            0 => "",
            1 => "#",
            -1 => "b",
            2 => "+",
            -2 => "&",
            n if n > 0 => &format!("({}#)", n),
            n => &format!("({}b)", -n),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Debug for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Accidental({})", self)
    }
}

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
}
