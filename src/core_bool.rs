use crate::Should;

impl<'a> Should<'a, bool> {
    /// Asserts that the value is true.
    ///
    /// # Examples
    /// ```
    /// use shoulds::Shouldable;
    ///
    /// let value = true;
    /// value.should().be_true();
    /// ```
    ///
    /// # Panics
    /// Panics if the actual value is not true.
    pub fn be_true(&self) {
        assert_eq!(
            *self.actual(),
            true,
            "Expected {:?} to be true",
            self.actual()
        );
    }

    /// Asserts that the value is false.
    ///
    /// # Examples
    /// ```
    /// use shoulds::Shouldable;
    ///
    /// let value = false;
    /// value.should().be_false();
    /// ```
    ///
    /// # Panics
    /// Panics if the actual value is not false.
    pub fn be_false(&self) {
        assert_eq!(
            *self.actual(),
            false,
            "Expected {:?} to be false",
            self.actual()
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::Shouldable;

    #[test]
    fn be_true_success_should_not_panic() {
        let value = true;
        value.should().be_true();
    }

    #[test]
    #[should_panic(expected = "Expected false to be true")]
    fn be_true_fail_should_panic() {
        let value = false;
        value.should().be_true();
    }

    #[test]
    fn be_false_success_should_not_panic() {
        let value = false;
        value.should().be_false();
    }

    #[test]
    #[should_panic(expected = "Expected true to be false")]
    fn be_false_fail_should_panic() {
        let value = true;
        value.should().be_false();
    }

    #[test]
    fn bool_expressions_with_be_true() {
        let a = 5;
        let b = 10;
        (a < b).should().be_true();
    }

    #[test]
    fn bool_expressions_with_be_false() {
        let a = 15;
        let b = 10;
        (a < b).should().be_false();
    }

    #[test]
    fn bool_variables_with_be_true() {
        let is_enabled = true;
        is_enabled.should().be_true();
    }

    #[test]
    fn bool_variables_with_be_false() {
        let is_enabled = false;
        is_enabled.should().be_false();
    }

    #[test]
    fn bool_from_function_with_be_true() {
        fn is_even(n: i32) -> bool {
            n % 2 == 0
        }

        is_even(4).should().be_true();
    }

    #[test]
    fn bool_from_function_with_be_false() {
        fn is_even(n: i32) -> bool {
            n % 2 == 0
        }

        is_even(3).should().be_false();
    }

    #[test]
    fn bool_from_method_with_be_true() {
        let text = "Hello, world!";
        text.contains("world").should().be_true();
    }

    #[test]
    fn bool_from_method_with_be_false() {
        let text = "Hello, world!";
        text.contains("universe").should().be_false();
    }

    #[test]
    fn bool_from_comparison_with_be_true() {
        let vec = vec![1, 2, 3];
        (vec.len() == 3).should().be_true();
    }

    #[test]
    fn bool_from_comparison_with_be_false() {
        let vec = vec![1, 2, 3];
        (vec.len() == 0).should().be_false();
    }
}
