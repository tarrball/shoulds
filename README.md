# shoulds

Shouldly-inspired assertions for Rust

## Installation

`Cargo.toml`:

```toml
[dependencies]
shoulds = { git = "https://github.com/tarrball/shoulds", branch = "main" }
```

## Use

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
