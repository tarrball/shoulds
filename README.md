# shoulds

[![Crates.io](https://img.shields.io/crates/v/shoulds)](https://crates.io/crates/shoulds)

**shoulds** is a fluent assertion library for Rust tests, inspired
by [FluentAssertions](https://github.com/fluentassertions/fluentassertions)
and [Shouldly](https://github.com/shouldly/shouldly).

It helps you write readable, intention-revealing tests like:

```rust
use shoulds::Shouldable;

#[test]
fn adds_numbers_correctly() {
    let result = 40 + 2;

    result.should().eq(&42);
}
```

The API is designed to be intuitive and discoverable, so you can
express expectations naturally—with minimal boilerplate and maximum clarity.

---

## ✨ Assertions

Currently supported:

### 🔢 Equality

- `should().eq(&expected)`
- `should().ne(&unexpected)`

### ✅ Booleans

- `should().be_true()`
- `should().be_false()`

### 🧩 Option

- `should().be_some()`
- `should().be_none()`

### 🧪 Result

- `should().be_ok()`
- `should().be_err()`

---

## 📦 Installation

In your `Cargo.toml`:

```toml
[dev-dependencies]
shoulds = "0.3.0"
```

---

## 🧪 Examples

```rust
use shoulds::Shouldable;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn it_adds_numbers() {
    let result = add(40, 2);
    result.should().eq(&42);
}
```

---

## 📄 License

MIT
