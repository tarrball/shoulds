# shoulds

_shoulds_ is a [Shouldly-inspired](https://docs.shouldly.io) test assertion library for testing Rust.

## Current Assertions:

### Equality

-   `should_be`
-   `should_be_greater_than`
-   `should_be_greater_than_or_equal_to`
-   `should_be_less_than`
-   `should_be_less_than_or_equal_to`

### Strings

-   `should_contain`
-   `should_not_contain`

### Booleans

-   `should_be_false`
-   `should_be_true`

### Results

-   `should_be_error`
-   `should_be_ok`

I'll add more assertions along the way as the need arises.

## Install:

`Cargo.toml`:

```toml
[dev-dependencies]
shoulds = "0.1.6"
```

## Enjoy:

`src/lib.rs`:

```rs
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn concat(a: &String, b: &String) -> String {
    format!("{}{}", a, b)
}

#[cfg(test)]
mod tests {
    use crate::{add, concat};
    use shoulds::*;

    #[test]
    fn add_adds_params_together() {
        let result = add(40, 2);

        result.should_be(42)
    }

    #[test]
    fn concat_concats_strings_together() {
        let result = concat(&"Hello, ".to_string(), &"world!".to_string());

        result.should_be("Hello, world!".to_string());
    }
}
```
