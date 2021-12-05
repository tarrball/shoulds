use std::fmt::Debug;

trait Shouldly<T: PartialEq + Debug>  {
    fn should_be(self, expected: T);

    fn should_not_be(self, expected: T);
}

trait ShouldlyStr {
    fn should_be(&self, expected: &str);

    fn should_not_be(&self, expected: &str);
}

impl<T: PartialEq + Debug> Shouldly<T> for T {
    fn should_be(self, expected: T) {
        assert_eq!(self, expected);
    }

    fn should_not_be(self, expected: T) {
        assert_ne!(self, expected);
    }
}

impl ShouldlyStr for str {
    fn should_be(&self, expected: &str) {
        assert_eq!(self, expected);
    }

    fn should_not_be(&self, expected: &str) {
        assert_ne!(self, expected);
    }
}

#[cfg(test)]
mod tests {
    use crate::Shouldly;

    #[test]
    fn i32_should_be() {
        2.should_be(2);
    }

    #[test]
    fn i32_should_not_be() {
        1.should_not_be(2);
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
