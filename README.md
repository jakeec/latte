# Latte

Ever wanted Mocha's `describe`/`it` syntax in Rust? No? Well here it is anyway! This crate simply contains two macros `describe!` and `it!` that expand into Rust's native test constructs.

## Example

```rust
describe!(test_suite, {
    it!(does_something, {
        assert_eq!(1, 1);
    });

    it!(does_something_else, {
        assert!(false);
    });
});
```

Is equivalent to

```rust
#[cfg(test)]
mod test_suite {
    #[test]
    fn does_something() {
        assert_eq!(1, 1);
    }

    #[test]
    fn does_something_else() {
        assert!(false);
    }
}
```

## Setup

Install the crate by adding `latte` to your Cargo.toml's dependencies and then simply add this import to your code:

```rust
#[macro_use]
extern crate latte;
```

## Purpose

There really is no purpose to this. It's arguably a bit easier to read because you don't have a bunch of attributes cluttering up the place, but ultimately this was just an exercise to learn the basics of `macro_rules!`. If you do happen to find this library somewhat useful and would like to see some features added, let me know on GitHub or submit a pull request.
