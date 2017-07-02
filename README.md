dow
===

Return the day of the week a specific date falls on (uses [Tomohiko Sakamoto's algorithm](https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Sakamoto.27s_methods)).

[![Build Status](https://travis-ci.org/neosilky/dow.svg?branch=master)](https://travis-ci.org/neosilky/dow)

- [Documentation](https://docs.rs/dow)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dow = "0.1"
```

and this to your crate root:

```rust
extern crate dow;
```

and then you can use it as follows:

```rust
let day_index = dow::dow(2017, 7, 2);
assert_eq!(day_index, 0);
```

Note: Sunday is `0`, Monday is `1` etc.
