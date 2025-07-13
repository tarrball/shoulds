use crate::Should;
use std::fmt::Debug;

impl<'a, T> Should<'a, T>
where
    T: PartialEq + Debug,
{
    /// Asserts that the value is not equal to the expected value.
    ///
    /// # Examples
    /// ```
    /// use shoulds::Shouldable;
    ///
    /// let value = 42;
    /// value.should().ne(&43);
    /// ```
    ///
    /// # Panics
    /// Panics if the actual value is equal to the expected value.
    pub fn ne(&self, expected: &T) {
        assert_ne!(
            self.actual(),
            expected,
            "Expected {:?} not to eq equal to {:?}",
            self.actual(),
            expected
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::Shouldable;

    #[test]
    fn ne_i32_success_should_not_panic() {
        let value = 42;
        value.should().ne(&43);
    }

    #[test]
    #[should_panic(expected = "Expected 42 not to eq equal to 42")]
    fn ne_i32_fail_should_panic() {
        let value = 42;
        value.should().ne(&42);
    }

    #[test]
    fn ne_f64_success_should_not_panic() {
        let value = 3.14;
        value.should().ne(&2.71);
    }

    #[test]
    #[should_panic(expected = "Expected 3.14 not to eq equal to 3.14")]
    fn ne_f64_fail_should_panic() {
        let value = 3.14;
        value.should().ne(&3.14);
    }

    #[test]
    fn ne_string_success_should_not_panic() {
        let value = "Hello".to_string();
        value.should().ne(&"World".to_string());
    }

    #[test]
    #[should_panic(expected = "Expected \"Hello\" not to eq equal to \"Hello\"")]
    fn ne_string_fail_should_panic() {
        let value = "Hello".to_string();
        value.should().ne(&"Hello".to_string());
    }

    #[test]
    fn ne_char_success_should_not_panic() {
        let value = 'a';
        value.should().ne(&'b');
    }

    #[test]
    #[should_panic(expected = "Expected 'a' not to eq equal to 'a'")]
    fn ne_char_fail_should_panic() {
        let value = 'a';
        value.should().ne(&'a');
    }

    #[test]
    fn ne_bool_success_should_not_panic() {
        let value = true;
        value.should().ne(&false);
    }

    #[test]
    #[should_panic(expected = "Expected true not to eq equal to true")]
    fn ne_bool_fail_should_panic() {
        let value = true;
        value.should().ne(&true);
    }

    #[test]
    fn ne_tuple_success_should_not_panic() {
        let value = (1, 2);
        value.should().ne(&(2, 3));
    }

    #[test]
    #[should_panic(expected = "Expected (1, 2) not to eq equal to (1, 2)")]
    fn ne_tuple_fail_should_panic() {
        let value = (1, 2);
        value.should().ne(&(1, 2));
    }

    #[test]
    fn ne_vector_success_should_not_panic() {
        let value = vec![1, 2, 3];
        value.should().ne(&vec![4, 5, 6]);
    }

    #[test]
    #[should_panic(expected = "Expected [1, 2, 3] not to eq equal to [1, 2, 3]")]
    fn ne_vector_fail_should_panic() {
        let value = vec![1, 2, 3];
        value.should().ne(&vec![1, 2, 3]);
    }

    #[test]
    fn ne_array_success_should_not_panic() {
        let value = [1, 2, 3];
        value.should().ne(&[4, 5, 6]);
    }

    #[test]
    #[should_panic(expected = "Expected [1, 2, 3] not to eq equal to [1, 2, 3]")]
    fn ne_array_fail_should_panic() {
        let value = [1, 2, 3];
        value.should().ne(&[1, 2, 3]);
    }

    #[test]
    fn ne_struct_success_should_not_panic() {
        #[derive(PartialEq, Debug)]
        struct MyStruct {
            value: i32,
        }

        let value = MyStruct { value: 1 };
        value.should().ne(&MyStruct { value: 2 });
    }

    #[test]
    #[should_panic(
        expected = "Expected MyStruct { value: 1 } not to eq equal to MyStruct { value: 1 }"
    )]
    fn ne_struct_fail_should_panic() {
        #[derive(PartialEq, Debug)]
        struct MyStruct {
            value: i32,
        }

        let value = MyStruct { value: 1 };
        value.should().ne(&MyStruct { value: 1 });
    }

    #[test]
    fn ne_custom_type_success_should_not_panic() {
        #[derive(PartialEq, Debug)]
        struct CustomType {
            id: i32,
            name: String,
        }

        let value = CustomType {
            id: 1,
            name: "Test".to_string(),
        };

        value.should().ne(&CustomType {
            id: 2,
            name: "Test2".to_string(),
        });
    }

    #[test]
    #[should_panic(
        expected = "Expected CustomType { id: 1, name: \"Test\" } not to eq equal to CustomType { id: 1, name: \"Test\" }"
    )]
    fn ne_custom_type_fail_should_panic() {
        #[derive(PartialEq, Debug)]
        struct CustomType {
            id: i32,
            name: String,
        }

        let value = CustomType {
            id: 1,
            name: "Test".to_string(),
        };

        value.should().ne(&CustomType {
            id: 1,
            name: "Test".to_string(),
        });
    }

    #[test]
    fn ne_enum_success_should_not_panic() {
        #[derive(PartialEq, Debug)]
        enum MyEnum {
            Variant1,
            Variant2,
        }

        let value = MyEnum::Variant1;
        value.should().ne(&MyEnum::Variant2);
    }

    #[test]
    #[should_panic(expected = "Expected Variant1 not to eq equal to Variant1")]
    fn ne_enum_fail_should_panic() {
        #[derive(PartialEq, Debug)]
        enum MyEnum {
            Variant1,
            Variant2,
        }

        let value1 = MyEnum::Variant1;
        value1.should().ne(&MyEnum::Variant1);

        let value2 = MyEnum::Variant2;
        value2.should().ne(&MyEnum::Variant2);
    }

    #[test]
    fn ne_option_success_should_not_panic() {
        let value = Some(42);
        value.should().ne(&None);
    }

    #[test]
    #[should_panic(expected = "Expected Some(42) not to eq equal to Some(42)")]
    fn ne_option_fail_should_panic() {
        let value = Some(42);
        value.should().ne(&Some(42));
    }

    #[test]
    fn ne_result_success_should_not_panic() {
        let value: Result<i32, &str> = Ok(42);
        value.should().ne(&Err("Error"));
    }

    #[test]
    #[should_panic(expected = "Expected Ok(42) not to eq equal to Ok(42)")]
    fn ne_result_fail_should_panic() {
        let value: Result<i32, &str> = Ok(42);
        value.should().ne(&Ok(42));
    }
}
