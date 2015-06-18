# Funky!

Funky allows you to fire any function that implements Sync + Send quickly into a
thread. The thread takes ownership of any arguments passed into the macro.

The macro returns a mpsc::Receiver on which you can block until you have the result.

# Installation

Add to your `Cargo.toml`

```
funky = "*"
```

And run `cargo install`

# Usage

Add to your main or lib file:

```rust
#[macro_use]
extern crate funky;
```

An example with no arguments:

```rust
let func = || -> String {
  "abc".to_string()
};

let rx = funky!(func);
assert_eq!(rx.recv().unwrap(), "abc".to_string())
```

And with many arguments:

```rust
let func = |string: String| -> String {
  string
};

let rx = funky!(func, "abc".to_string());
assert_eq!(rx.recv().unwrap(), "abc".to_string())
```
