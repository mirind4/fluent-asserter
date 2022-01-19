use super::*;
use std::ops::Neg;
use num::Unsigned;
use num::traits::Pow;
use num::{traits::Zero, Integer, Signed, Bounded, Float}; // 0.2.0

macro_rules! abs_diff_unsigned {
    ($x:expr, $y:expr, $d:expr) => {
        if (if $x > $y {
            $x - $y 
        } else {
            $y - $x
        }) > $d {
            panic!("AssertionError: not equal");
        }
    }
}

macro_rules! abs_diff_eq {
    ($x:expr, $y:expr, $d:expr) => {
        if (($x - $y).abs() > $d) { panic!("AssertionError: not equal"); }
    }
}

macro_rules! abs_diff {
    ($x:expr, $y:expr) => {
        ($x - $y).abs()
    }
}

pub trait ApproxEqualMarkerTrait {}
struct UnsignedIntApproxEqual;
struct SignedIntApproxEqual;
struct FloatApproxEqual;

impl ApproxEqualMarkerTrait for SignedIntApproxEqual{}
impl ApproxEqualMarkerTrait for FloatApproxEqual{}
impl ApproxEqualMarkerTrait for UnsignedIntApproxEqual{}

trait ApproximatelyEqual<T, S:ApproxEqualMarkerTrait  > {
    fn is_approx_equal_to(self, expected: T, delta: T);
}


impl<T> ApproximatelyEqual<T, UnsignedIntApproxEqual> for Asserter<T> where T : Unsigned + Integer + Zero  + Bounded + Copy {
    fn is_approx_equal_to(self, expected: T, delta: T) {
        abs_diff_unsigned!(self.value,expected,delta);
    }
}

impl<T> ApproximatelyEqual<T, SignedIntApproxEqual> for Asserter<T> where T : Signed + Integer + Zero + Neg + Bounded + Copy {
    fn is_approx_equal_to(self, expected: T, delta: T) {
        abs_diff_eq!(self.value,expected,delta);
    }
}

impl<T> ApproximatelyEqual<T,FloatApproxEqual> for Asserter<T> where T :Float + Zero + Neg + Copy + std::fmt::Display{
    fn is_approx_equal_to(self, expected: T, delta: T) {
        let rounder = 10f64.pow(get_length_of_rounder(delta));
        
        let diff = abs_diff!(self.value,expected);

        let diff_f64 = round(diff, rounder);
        let delta_f64 =  round(delta, rounder);

        if diff_f64 > delta_f64 {
            panic!("AssertionError: Not equal")
        }
    }
}


//TODO: add abstraction for this, either in a new struct with new or some logic class
fn get_length_of_rounder<T>(delta: T) -> f64 where T: ToString {
    return delta.to_string()
        .split(".")
        .last()
        .unwrap()
        .len()
        .to_string()
        .parse()
        .unwrap();
}

fn round<T>(diff: T, rounder: f64) -> f64 where T: std::fmt::Display {
    let number : f64 = format!("{}",diff).parse().unwrap();
    let number: f64 = (number * rounder).round() / rounder;
    
    number
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::panic_asserter_helper::assert_that_panics;

    #[test]
    fn test_is_equal_to_approximately_for_unsigned() {
        assert_that!(3u32).is_approx_equal_to(2,1);
        assert_that!(3u32).is_approx_equal_to(3,1);
        assert_that!(3u32).is_approx_equal_to(4,1);

        assert_that_panics(||assert_that!(3u32).is_approx_equal_to(5,1));

    }
    
    #[test]
    fn test_is_equal_to_approximately_for_signed() {
        assert_that!(3i32).is_approx_equal_to(2,1);
        assert_that!(3i32).is_approx_equal_to(3,1);
        assert_that!(3i32).is_approx_equal_to(4,1);

        assert_that_panics(||assert_that!(3i32).is_approx_equal_to(5,1));

    }
    
    #[test]
    fn test_is_equal_to_approximately_for_floats() {
        assert_that!(3.14).is_approx_equal_to(3.16,0.02);
        assert_that!(3.14).is_approx_equal_to(3.14,0.00);
        assert_that!(3.14159).is_approx_equal_to(3.14157,0.00002);

        assert_that_panics(||assert_that!(3.14159).is_approx_equal_to(3.14157,0.00001));
    }
}