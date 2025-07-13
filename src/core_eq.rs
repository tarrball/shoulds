use crate::Should;
use std::fmt::Debug;

impl<'a, T> Should<'a, T>
where
    T: PartialEq + Debug,
{
    /// Asserts that the value is equal to the expected value.
    ///
    /// # Examples
    /// ```
    /// use shoulds::Shouldable;
    ///
    /// let value = 42;
    /// value.should().eq(&42);
    /// ```
    ///
    /// # Panics
    /// Panics if the actual value is not equal to the expected value.
    pub fn eq(&self, expected: &T) {
        assert_eq!(
            self.actual(),
            expected,
            "Expected {:?} to equal to {:?}",
            self.actual(),
            expected
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::Shouldable;

    #[test]
    fn eq_i32_success_should_not_panic() {
        let value = 42;
        value.should().eq(&42);
    }

    #[test]
    #[should_panic(expected = "Expected 42 to equal to 43")]
    fn eq_i32_fail_should_panic() {
        let value = 42;
        value.should().eq(&43);
    }

    #[test]
    fn eq_f64_success_should_not_panic() {
        let value = 3.14;
        value.should().eq(&3.14);
    }

    #[test]
    #[should_panic(expected = "Expected 3.14 to equal to 2.71")]
    fn eq_f64_fail_should_panic() {
        let value = 3.14;
        value.should().eq(&2.71);
    }

    #[test]
    fn eq_string_success_should_not_panic() {
        let value = "Hello".to_string();
        value.should().eq(&"Hello".to_string());
    }

    #[test]
    #[should_panic(expected = "Expected \"Hello\" to equal to \"World\"")]
    fn eq_string_fail_should_panic() {
        let value = "Hello".to_string();
        value.should().eq(&"World".to_string());
    }

    #[test]
    fn eq_char_success_should_not_panic() {
        let value = 'a';
        value.should().eq(&'a');
    }

    #[test]
    #[should_panic(expected = "Expected 'a' to equal to 'b'")]
    fn eq_char_fail_should_panic() {
        let value = 'a';
        value.should().eq(&'b');
    }

    #[test]
    fn eq_bool_success_should_not_panic() {
        let value = true;
        value.should().eq(&true);
    }

    #[test]
    #[should_panic(expected = "Expected true to equal to false")]
    fn eq_bool_fail_should_panic() {
        let value = true;
        value.should().eq(&false);
    }

    #[test]
    fn eq_tuple_success_should_not_panic() {
        let value = (1, 2);
        value.should().eq(&(1, 2));
    }

    #[test]
    #[should_panic(expected = "Expected (1, 2) to equal to (3, 4)")]
    fn eq_tuple_fail_should_panic() {
        let value = (1, 2);
        value.should().eq(&(3, 4));
    }

    #[test]
    fn eq_vector_success_should_not_panic() {
        let value = vec![1, 2, 3];
        value.should().eq(&vec![1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "Expected [1, 2, 3] to equal to [4, 5, 6]")]
    fn eq_vector_fail_should_panic() {
        let value = vec![1, 2, 3];
        value.should().eq(&vec![4, 5, 6]);
    }

    #[test]
    fn eq_array_success_should_not_panic() {
        let value = [1, 2, 3];
        value.should().eq(&[1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "Expected [1, 2, 3] to equal to [4, 5, 6]")]
    fn eq_array_fail_should_panic() {
        let value = [1, 2, 3];
        value.should().eq(&[4, 5, 6]);
    }

    #[test]
    fn eq_struct_success_should_not_panic() {
        #[derive(PartialEq, Debug)]
        struct MyStruct {
            a: i32,
            b: String,
        }

        let value = MyStruct {
            a: 1,
            b: "test".to_string(),
        };

        value.should().eq(&MyStruct {
            a: 1,
            b: "test".to_string(),
        });
    }

    #[test]
    #[should_panic(
        expected = "Expected MyStruct { a: 1, b: \"test\" } to equal to MyStruct { a: 2, b: \"test\" }"
    )]
    fn eq_struct_fail_should_panic() {
        #[derive(PartialEq, Debug)]
        struct MyStruct {
            a: i32,
            b: String,
        }

        let value = MyStruct {
            a: 1,
            b: "test".to_string(),
        };

        value.should().eq(&MyStruct {
            a: 2,
            b: "test".to_string(),
        });
    }

    #[test]
    fn eq_custom_type_success_should_not_panic() {
        #[derive(PartialEq, Debug)]
        struct CustomType {
            value: i32,
        }

        let value = CustomType { value: 10 };
        value.should().eq(&CustomType { value: 10 });
    }

    #[test]
    #[should_panic(
        expected = "Expected CustomType { value: 10 } to equal to CustomType { value: 20 }"
    )]
    fn eq_custom_type_fail_should_panic() {
        #[derive(PartialEq, Debug)]
        struct CustomType {
            value: i32,
        }

        let value = CustomType { value: 10 };
        value.should().eq(&CustomType { value: 20 });
    }

    #[test]
    fn eq_enum_success_should_not_panic() {
        #[derive(PartialEq, Debug)]
        enum MyEnum {
            Variant1,
            Variant2(i32),
        }

        let value1 = MyEnum::Variant1;
        value1.should().eq(&MyEnum::Variant1);

        let value2 = MyEnum::Variant2(42);
        value2.should().eq(&MyEnum::Variant2(42));
    }

    #[test]
    #[should_panic(expected = "Expected Variant1 to equal to Variant2(42)")]
    fn eq_enum_fail_should_panic() {
        #[derive(PartialEq, Debug)]
        enum MyEnum {
            Variant1,
            Variant2(i32),
        }

        let value = MyEnum::Variant1;
        value.should().eq(&MyEnum::Variant2(42));
    }

    #[test]
    fn eq_option_success_should_not_panic() {
        let value = Some(42);
        value.should().eq(&Some(42));
    }

    #[test]
    #[should_panic(expected = "Expected Some(42) to equal to None")]
    fn eq_option_fail_should_panic() {
        let value = Some(42);
        value.should().eq(&None);
    }

    #[test]
    fn eq_result_success_should_not_panic() {
        let value: Result<i32, &str> = Ok(42);
        value.should().eq(&Ok(42));
    }

    #[test]
    #[should_panic(expected = "Expected Ok(42) to equal to Err(\"error\")")]
    fn eq_result_fail_should_panic() {
        let value: Result<i32, &str> = Ok(42);
        value.should().eq(&Err("error"));
    }
}
