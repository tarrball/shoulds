# shoulds

[![Crates.io](https://img.shields.io/crates/v/shoulds)](https://crates.io/crates/shoulds)

**shoulds** is a fluent assertion library for Rust tests, inspired
by [FluentAssertions](https://github.com/fluentassertions/fluentassertions)
and [Shouldly](https://github.com/shouldly/shouldly).

It helps you write readable, intention-revealing tests like:

```rust
use shoulds::Shouldable;

let result = 42;
result.should().eq( & 42);
```

---

## ✨ Assertions

Currently supported:

### ✅ Equality

- `should().eq(&expected)`
- `should().ne(&unexpected)`

More comparison methods (greater than, less than, etc.) are coming soon.

---

## 📦 Installation

In your `Cargo.toml`:

```toml
[dev-dependencies]
shoulds = "0.2.0"
```

---

## 🧪 Example

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

## 🚧 Roadmap

Planned enhancements include:

- String and collection assertions (`contain`, `have_length`, etc.)
- Result/Option assertions (`be_ok`, `be_some`)
- Custom diffing for better failure output
- Optional colored output
- Snapshot support?

Want to help shape this crate? Feedback and contributions welcome!

---

## ⚠️ Note

`shoulds` is still experimental and evolving. Expect breaking changes in the 0.x versions.

---

## 📄 License

MIT
