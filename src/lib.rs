//! # shoulds
//!
//! `shoulds` is a fluent assertion library for writing expressive and readable test assertions in Rust.
//!
//! Inspired by [FluentAssertions](https://github.com/fluentassertions/fluentassertions) and [Shouldly](https://github.com/shouldly/shouldly),
//! this crate provides a `.should()` extension that enables fluent chaining of assertions.
//!
//! ## Example
//!
//! ```rust
//! use shoulds::Shouldable;
//!
//! let value = 42;
//! value.should().eq(&42);
//! ```
//!
//! ## Status
//!
//! This crate is experimental and evolving. The API may change in 0.x versions as we build toward a richer set of assertions and improved error messages.

mod core_eq;
mod core_ne;
mod should;

pub use should::{Should, Shouldable};
