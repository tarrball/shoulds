use crate::Should;
use std::fmt::Debug;

impl<'a, T, E> Should<'a, Result<T, E>>
where
    T: Debug,
    E: Debug,
{
    /// Asserts that the Result is Ok.
    ///
    /// # Examples
    /// ```
    /// use shoulds::Shouldable;
    ///
    /// let result: Result<i32, &str> = Ok(42);
    /// result.should().be_ok();
    /// ```
    ///
    /// # Panics
    /// Panics if the Result is an Err.
    pub fn be_ok(&self) {
        assert!(
            self.actual().is_ok(),
            "Expected Result to be Ok, but got Err({:?})",
            self.actual().as_ref().err()
        )
    }

    /// Asserts that the Result is Err.
    ///
    /// # Examples
    /// ```
    /// use shoulds::Shouldable;
    ///
    /// let result: Result<i32, &str> = Err("error");
    /// result.should().be_err();
    /// ```
    ///
    /// # Panics
    /// Panics if the Result is Ok.
    pub fn be_err(&self) {
        assert!(
            self.actual().is_err(),
            "Expected Result to be Err, but got Ok({:?})",
            self.actual().as_ref().ok()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Shouldable;

    #[test]
    fn be_ok_success_should_not_panic() {
        let result: Result<i32, &str> = Ok(42);
        result.should().be_ok();
    }

    #[test]
    #[should_panic(expected = "Expected Result to be Ok, but got Err")]
    fn be_ok_fail_should_panic() {
        let result: Result<i32, &str> = Err("error");
        result.should().be_ok();
    }

    #[test]
    fn be_err_success_should_not_panic() {
        let result: Result<i32, &str> = Err("error");
        result.should().be_err();
    }

    #[test]
    #[should_panic(expected = "Expected Result to be Err, but got Ok")]
    fn be_err_fail_should_panic() {
        let result: Result<i32, &str> = Ok(42);
        result.should().be_err();
    }
}
