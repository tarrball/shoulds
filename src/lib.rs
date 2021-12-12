//! # shoulds
//! 
//! `shoulds` is an assertion library for writing easy and enjoyable test assertions.

use std::fmt::Debug;

pub trait ShouldsBool {
    /// Asserts that `self` is false.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result = false;
    /// 
    /// test_result.should_be_false();
    /// ```
    /// 
    /// # Panics 
    /// 
    /// Panics if `self` is true.
    fn should_be_false(self);

    /// Asserts that `self` is true.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result = true;
    /// 
    /// test_result.should_be_true();
    /// ```
    /// 
    /// # Panics 
    /// 
    /// Panics if `self` is false.
    fn should_be_true(self);
}

impl ShouldsBool for bool {
    fn should_be_false(self) {
        assert_eq!(self, false);
    }

    fn should_be_true(self) {
        assert_eq!(self, true);
    }
}

pub trait ShouldsEq<T: PartialEq + Debug>  {
    /// Asserts that `self` is equal to `expected`.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result = 42;
    /// let expected = 42;
    /// 
    /// test_result.should_be(expected);
    /// ```
    /// 
    /// # Panics 
    /// 
    /// Panics if `self` is not equal to `expected`.
    fn should_be(self, expected: T);

    /// Asserts that `self` is not equal to `expected`.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result = 42;
    /// let expected = 9001;
    /// 
    /// test_result.should_not_be(expected);
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if `self` is equal to `expected`.
    fn should_not_be(self, expected: T);
}

impl<T: PartialEq + Debug> ShouldsEq<T> for T {
    fn should_be(self, expected: T) {
        assert_eq!(self, expected);
    }

    fn should_not_be(self, expected: T) {
        assert_ne!(self, expected);
    }
}

pub trait ShouldsOrds<T: PartialOrd + Debug>  {    
    /// Asserts that `self` is greater than `expected`.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result = 9001;
    /// let expected = 9000;
    /// 
    /// test_result.should_be_greater_than(expected);
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if `self` is less than or equal to `expected`.
    fn should_be_greater_than(self, expected: T);

    /// Asserts that `self` is greater than or equal to `expected`.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result = 9001;
    /// let expected = 9000;
    /// 
    /// test_result.should_be_greater_than_or_equal_to(expected);
    /// expected.should_be_greater_than_or_equal_to(expected);
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if `self` is less than `expected`.
    fn should_be_greater_than_or_equal_to(self, expected: T);

    /// Asserts that `self` is less than `expected`.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result = 9000;
    /// let expected = 9001;
    /// 
    /// test_result.should_be_less_than(expected);
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if `self` is greater than or equal to `expected`.
    fn should_be_less_than(self, expected: T);

    /// Asserts that `self` is less than or equal to `expected`.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result = 9000;
    /// let expected = 9001;
    /// 
    /// test_result.should_be_less_than_or_equal_to(expected);
    /// test_result.should_be_less_than_or_equal_to(test_result);
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if `self` is greater than `expected`.
    fn should_be_less_than_or_equal_to(self, expected: T);
}

impl<T: PartialOrd + Debug> ShouldsOrds<T> for T {
    fn should_be_greater_than(self, expected: T) {
        assert!(self > expected);
    }

    fn should_be_greater_than_or_equal_to(self, expected: T) {
        assert!(self >= expected);
    }

    fn should_be_less_than(self, expected: T) {
        assert!(self < expected);
    }

    fn should_be_less_than_or_equal_to(self, expected: T) {
        assert!(self <= expected);
    }
}

pub trait ShouldsStr {
    /// Asserts that `&self` is equal to `&expected`.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result = String::from("Hello, world!");
    /// let expected = String::from("Hello, world!");
    /// 
    /// test_result.should_be(expected);
    /// ```
    /// 
    /// # Panics 
    /// 
    /// Panics if `&self` is not equal to `&expected`.
    fn should_be(&self, expected: &str);

    /// Asserts that `&self` is not equal to `&expected`.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result = String::from("Hello, world!");
    /// let expected = String::from("World, hello!");
    /// 
    /// test_result.should_not_be(expected);
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if `self` is equal to `expected`.
    fn should_not_be(&self, expected: &str);
}

impl ShouldsStr for str {
    fn should_be(&self, expected: &str) {
        assert_eq!(self, expected);
    }

    fn should_not_be(&self, expected: &str) {
        assert_ne!(self, expected);
    }
}

pub trait ShouldsResult<T, E> {
    /// Asserts that `&self` is a success.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result: Result<i32, String> = Ok(42);
    /// 
    /// test_result.should_be_ok();
    /// ```
    /// 
    /// # Panics 
    /// 
    /// Panics if `&self` is a failure.
    fn should_be_ok(&self);

    /// Asserts that `&self` is a failure.
    /// 
    /// # Examples
    /// ```
    /// use shoulds::*;
    /// 
    /// let test_result: Result<i32, String> = Err(String::from("Oh dear!"));
    /// 
    /// test_result.should_be_error();
    /// ```
    /// 
    /// # Panics 
    /// 
    /// Panics if `&self` is an `Err` result.
    fn should_be_error(&self);
}

impl<T, E> ShouldsResult<T, E> for Result<T, E> {
    fn should_be_error(&self) {
        assert_eq!(self.is_err(), true);
    }

    fn should_be_ok(&self) {
        assert_eq!(self.is_ok(), true);
    }
}

#[cfg(test)]
mod tests {
    use crate::{ShouldsBool, ShouldsResult, ShouldsOrds};

    #[test]
    #[should_panic]
    fn true_should_not_be_false() {
        true.should_be_false();
    }

    #[test]
    #[should_panic]
    fn false_should_not_be_true() {
        false.should_be_true();
    }

    #[test]
    #[should_panic]
    fn partialord_should_be_greater_than_lt_fail() {
        5.should_be_greater_than(6);
    }

    #[test]
    #[should_panic]
    fn partialord_should_be_greater_than_eq_fail() {
        5.should_be_greater_than(5);
    }

    #[test]
    #[should_panic]
    fn partialord_should_be_greater_than_or_equal_to_lt_fail() {
        5.should_be_greater_than(6);
    }

    #[test]
    #[should_panic]
    fn partialord_should_be_less_than_gt_fail() {
        5.should_be_less_than(4);
    }

    #[test]
    #[should_panic]
    fn partialord_should_be_less_than_eq_fail() {
        5.should_be_less_than(5);
    }

    #[test]
    #[should_panic]
    fn partialord_should_be_less_than_or_equal_to_gt_fail() {
        5.should_be_less_than_or_equal_to(4);
    }

    #[test]
    #[should_panic]
    fn error_result_should_not_be_ok() {
        let result: Result<i32, String> = Err("Whoops".to_string());
        
        result.should_be_ok();
    }

    #[test]
    #[should_panic]
    fn ok_result_should_not_be_error() {
        let result: Result<i32, String> = Ok(42);
        
        result.should_be_error();
    }
}
