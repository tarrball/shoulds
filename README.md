# shoulds

_shoulds_ is a [Shouldly-inspired](https://docs.shouldly.io) test assertion library for testing Rust.

Current assertion functions include:

* `should_be`
* `should_not_be`
* `should_be_false`
* `should_be_true`
* `should_be_error`
* `should_be_ok`

I'll add more assertions along the way as the need arises.

## Install:

`Cargo.toml`:

```toml
[dependencies.shoulds]
git = "https://github.com/tarrball/shoulds"
branch = "main"
```

## And Enjoy:

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
