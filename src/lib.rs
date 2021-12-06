use std::fmt::Debug;

pub trait Shoulds<T: PartialEq + Debug>  {
    fn should_be(self, expected: T);

    fn should_not_be(self, expected: T);
}

impl<T: PartialEq + Debug> Shoulds<T> for T {
    fn should_be(self, expected: T) {
        assert_eq!(self, expected);
    }

    fn should_not_be(self, expected: T) {
        assert_ne!(self, expected);
    }
}

pub trait ShouldsOrds<T: PartialOrd + Debug>  {    
    fn should_be_greater_than(self, expected: T);

    fn should_be_greater_than_or_equal_to(self, expected: T);

    fn should_be_less_than(self, expected: T);

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
    fn should_be(&self, expected: &str);

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

pub trait ShouldsBool {
    fn should_be_false(self);

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

pub trait ShouldsResult<T, E> {
    fn should_be_ok(&self);

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
    use crate::{Shoulds, ShouldsBool, ShouldsResult, ShouldsOrds};

    #[test]
    fn true_should_be_true() {
        true.should_be_true();
    }

    #[test]
    #[should_panic]
    fn true_should_not_be_false() {
        true.should_be_false();
    }

    #[test]
    fn false_should_be_false() {
        false.should_be_false();
    }

    #[test]
    #[should_panic]
    fn false_should_not_be_true() {
        false.should_be_true();
    }

    #[test]
    fn partialeq_should_be() {
        2.should_be(2);
    }

    #[test]
    fn partialeq_should_not_be() {
        1.should_not_be(2);
    }

    #[test]
    fn partialord_should_be_greater_than_pass() {
        5.should_be_greater_than(4);
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
    fn partialord_should_be_greater_than_or_equal_to_eq_pass() {
        5.should_be_greater_than_or_equal_to(5);
    }

    #[test]
    fn partialord_should_be_greater_than_or_equal_to_gt_pass() {
        5.should_be_greater_than_or_equal_to(4);
    }

    #[test]
    #[should_panic]
    fn partialord_should_be_greater_than_or_equal_to_lt_fail() {
        5.should_be_greater_than(6);
    }

    #[test]
    fn partialord_should_be_less_than_pass() {
        4.should_be_less_than(5);
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
    fn partialord_should_be_less_than_or_equal_to_eq_pass() {
        5.should_be_less_than_or_equal_to(5);
    }

    #[test]
    fn partialord_should_be_less_than_or_equal_to_gt_pass() {
        5.should_be_less_than_or_equal_to(6);
    }

    #[test]
    #[should_panic]
    fn partialord_should_be_less_than_or_equal_to_gt_fail() {
        5.should_be_less_than_or_equal_to(4);
    }


    #[test]
    fn error_result_should_be_error() {
        let result: Result<i32, String> = Err("Whoops".to_string());
        
        result.should_be_error();
    }

    #[test]
    #[should_panic]
    fn error_result_should_not_be_ok() {
        let result: Result<i32, String> = Err("Whoops".to_string());
        
        result.should_be_ok();
    }

    #[test]
    fn ok_result_should_be_ok() {
        let result: Result<i32, String> = Ok(42);
        
        result.should_be_ok();
    }

    #[test]
    #[should_panic]
    fn ok_result_should_not_be_error() {
        let result: Result<i32, String> = Ok(42);
        
        result.should_be_error();
    }

    #[test]
    fn string_should_be() {
        String::from("Hey").should_be(String::from("Hey"))
    }

    #[test]
    fn string_should_not_be() {
        String::from("Hey").should_not_be(String::from("What?"))
    }

    #[test]
    fn str_should_be() {
        "Hey".should_be("Hey");
    }

    #[test]
    fn str_should_not_be() {
        "Hey".should_not_be("What?");
    }
}
