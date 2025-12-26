#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    unreachable_pub,
    unused_qualifications,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
#![doc = include_str!("../README.md")]

pub mod harmony;

/// returns a, b such that a*y + b = x and 0 <= b < y
/// panics on y<=0, since this function is only needed for positive y
#[inline]
fn div_remainder(x: i16, y: i16) -> (i16, i16) {
    assert!(y > 0, "used div_remainder with nonpositive denominator");
    let q = x / y;
    let r = x % y;
    if r >= 0 { (q, r) } else { (q - 1, r + y) }
}

#[cfg(test)]
mod test {
    use super::div_remainder;
    fn div_test(x: i16, y: i16) {
        let (a, b) = div_remainder(x, y);
        assert!(0 <= b && b < y, "remainder was out of range");
        assert_eq!(a * y + b, x, "result was incorrect");
    }
    #[test]
    fn div() {
        // // just a sanity check
        // assert_eq!(7 / 3, 2);
        // assert_eq!(7 % 3, 1);
        // assert_eq!(-7 / 3, -2);
        // assert_eq!(-7 % 3, -1);

        div_test(1, 1);
        div_test(-12, 5);
        div_test(12, 5);
        div_test(5, 5);
        div_test(13, 8);
        div_test(13, 32);
        div_test(0, 32);
        div_test(32 + 13, 32);
    }
}
