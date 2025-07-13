/// A wrapper type that provides fluent-style assertions for a given value.
///
/// Constructed using the [`Shouldable::should`] method.
pub struct Should<'a, T> {
    actual: &'a T,
}

/// Returns a reference to the underlying value.
///
/// This can be useful for writing custom assertions or debugging.
impl<'a, T> Should<'a, T> {
    pub fn actual(&self) -> &T {
        self.actual
    }
}

/// A trait implemented for all types to enable fluent assertions using [`Should<T>`].
///
/// Call `.should()` on any value to access fluent assertion methods:
///
/// ```
/// use shoulds::Shouldable;
///
/// let x = 42;
/// x.should().eq(&42);
/// ```
pub trait Shouldable<T> {
    fn should(&self) -> Should<T>;
}

impl<T> Shouldable<T> for T {
    fn should(&self) -> Should<T> {
        Should { actual: self }
    }
}
