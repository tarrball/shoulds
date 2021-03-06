//! # shoulds
//!
//! `shoulds` is an assertion library for writing easy and enjoyable test assertions.

use std::fmt::{Debug, Display};

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
        assert!(self == false, "self should be false but was true");
    }

    fn should_be_true(self) {
        assert!(self == true, "self should be true but was false");
    }
}

pub trait ShouldsEq<T: PartialEq + Debug> {
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

impl<T: PartialEq + Debug + Display> ShouldsEq<T> for T {
    fn should_be(self, expected: T) {
        assert!(
            self == expected,
            "self should be {} but was {}",
            expected,
            self,
        );
    }

    fn should_not_be(self, expected: T) {
        assert!(
            self != expected,
            "self should not be {} but was {}",
            expected,
            self,
        );
    }
}

pub trait ShouldsOrds<T: PartialOrd + Debug> {
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

impl<T: PartialOrd + Debug + Display> ShouldsOrds<T> for T {
    fn should_be_greater_than(self, expected: T) {
        assert!(
            self > expected,
            "self should be greater than {} but was {}",
            expected,
            self
        );
    }

    fn should_be_greater_than_or_equal_to(self, expected: T) {
        assert!(
            self >= expected,
            "self should be greater than or equal to {} but was {}",
            expected,
            self
        );
    }

    fn should_be_less_than(self, expected: T) {
        assert!(
            self < expected,
            "self should be less than {} but was {}",
            expected,
            self
        );
    }

    fn should_be_less_than_or_equal_to(self, expected: T) {
        assert!(
            self <= expected,
            "self should be less than or equal to {} but was {}",
            expected,
            self
        );
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

    /// Asserts that `&self` contains `&query`.
    ///
    /// # Examples
    /// ```
    /// use shoulds::*;
    ///
    /// let text = String::from("Hello, world!");
    /// let query = String::from("world");
    ///
    /// text.should_contain(&query);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `&self` is does not contain `&query`.
    fn should_contain(&self, query: &str);

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
    /// Panics if `&self` is equal to `&expected`.
    fn should_not_be(&self, expected: &str);

    /// Asserts that `&self` does not contain `&query`.
    ///
    /// # Examples
    /// ```
    /// use shoulds::*;
    ///
    /// let text = String::from("Hello, world!");
    /// let query = String::from("woof");
    ///
    /// text.should_not_contain(&query);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `&self` is contains `&query`.
    fn should_not_contain(&self, query: &str);
}

impl ShouldsStr for str {
    fn should_be(&self, expected: &str) {
        assert!(
            self == expected,
            "self should be {} but was {}",
            expected,
            self
        );
    }

    fn should_contain(&self, query: &str) {
        assert!(
            self.contains(query),
            "self should contain '{}' but did not",
            query
        );
    }

    fn should_not_be(&self, expected: &str) {
        assert!(
            self != expected,
            "self should not be {} but was {}",
            expected,
            self
        );
    }

    fn should_not_contain(&self, query: &str) {
        assert!(
            !self.contains(query),
            "self should not contain '{}' but did",
            query
        );
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
        assert!(self.is_err() == true, "self should be err but was ok");
    }

    fn should_be_ok(&self) {
        assert!(self.is_ok() == true, "self should be ok but was err");
    }
}

#[cfg(test)]
mod tests {
    use crate::{ShouldsBool, ShouldsEq, ShouldsOrds, ShouldsResult, ShouldsStr};

    #[test]
    #[should_panic = "self should be false but was true"]
    fn true_should_not_be_false() {
        true.should_be_false();
    }

    #[test]
    #[should_panic = "self should be true but was false"]
    fn false_should_not_be_true() {
        false.should_be_true();
    }

    #[test]
    #[should_panic = "self should be 5 but was 6"]
    fn partialeq_should_be_fail() {
        6.should_be(5);
    }

    #[test]
    #[should_panic = "self should not be 5 but was 5"]
    fn partialeq_should_not_be_fail() {
        5.should_not_be(5);
    }

    #[test]
    #[should_panic = "self should be greater than 6 but was 5"]
    fn partialord_should_be_greater_than_lt_fail() {
        5.should_be_greater_than(6);
    }

    #[test]
    #[should_panic = "self should be greater than 5 but was 5"]
    fn partialord_should_be_greater_than_eq_fail() {
        5.should_be_greater_than(5);
    }

    #[test]
    #[should_panic = "self should be greater than or equal to 6 but was 5"]
    fn partialord_should_be_greater_than_or_equal_to_lt_fail() {
        5.should_be_greater_than_or_equal_to(6);
    }

    #[test]
    #[should_panic = "self should be less than 4 but was 5"]
    fn partialord_should_be_less_than_gt_fail() {
        5.should_be_less_than(4);
    }

    #[test]
    #[should_panic = "self should be less than 5 but was 5"]
    fn partialord_should_be_less_than_eq_fail() {
        5.should_be_less_than(5);
    }

    #[test]
    #[should_panic = "self should be less than or equal to 4 but was 5"]
    fn partialord_should_be_less_than_or_equal_to_gt_fail() {
        5.should_be_less_than_or_equal_to(4);
    }

    #[test]
    #[should_panic = "self should be ok but was err"]
    fn result_error_should_not_be_ok() {
        let result: Result<i32, String> = Err("Whoops".to_string());

        result.should_be_ok();
    }

    #[test]
    #[should_panic = "self should be err but was ok"]
    fn result_ok_should_not_be_error() {
        let result: Result<i32, String> = Ok(42);

        result.should_be_error();
    }

    #[test]
    #[should_panic = "self should contain 'goodbye' but did not"]
    fn str_contain_fail() {
        "hello".should_contain("goodbye");
    }

    #[test]
    #[should_panic = "self should not contain 'hello' but did"]
    fn str_should_not_contain_fail() {
        "hello".should_not_contain("hello");
    }

    #[test]
    #[should_panic = "self should contain 'goodbye' but did not"]
    fn string_contain_fail() {
        let text = String::from("hello");
        let query = String::from("goodbye");

        text.should_contain(&query)
    }

    #[test]
    #[should_panic = "self should not contain 'hello' but did"]
    fn string_should_not_contain_fail() {
        let text = String::from("hello");
        let query = String::from("hello");

        text.should_not_contain(&query)
    }
}
