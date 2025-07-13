use crate::Should;
use std::fmt::Debug;

impl<'a, T> Should<'a, Option<T>>
where
    T: Debug,
{
    /// Asserts that the Option is Some.
    ///
    /// # Examples
    /// ```
    /// use shoulds::Shouldable;
    ///
    /// let value: Option<i32> = Some(42);
    /// value.should().be_some();
    /// ```
    ///
    /// # Panics
    /// Panics if the Option is None.
    pub fn be_some(&self) {
        assert!(
            self.actual().is_some(),
            "Expected Option to be Some, but got None"
        )
    }

    /// Asserts that the Option is None.
    ///
    /// # Examples
    /// ```
    /// use shoulds::Shouldable;
    ///
    /// let value: Option<i32> = None;
    /// value.should().be_none();
    /// ```
    ///
    /// # Panics
    /// Panics if the Option is Some.
    pub fn be_none(&self) {
        assert!(
            self.actual().is_none(),
            "Expected Option to be None, but got Some({:?})",
            self.actual()
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::Shouldable;

    #[test]
    fn be_some_success_should_not_panic() {
        let value: Option<i32> = Some(42);
        value.should().be_some();
    }

    #[test]
    #[should_panic(expected = "Expected Option to be Some, but got None")]
    fn be_some_fail_should_panic() {
        let value: Option<i32> = None;
        value.should().be_some();
    }

    #[test]
    fn be_none_success_should_not_panic() {
        let value: Option<i32> = None;
        value.should().be_none();
    }

    #[test]
    #[should_panic(expected = "Expected Option to be None, but got Some")]
    fn be_none_fail_should_panic() {
        let value: Option<i32> = Some(42);
        value.should().be_none();
    }
}
