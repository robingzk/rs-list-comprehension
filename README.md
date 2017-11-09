# rs-list-comprehension - List Comprehension for Rust

Provides basic list comprehension for Rust as a macro. List comprehension is similar to using `filter` and `map` but provides a clearer way to write code in some cases.

### Usage
Add the crate as a dependency and use tha macro `lc`.
```rust
lc!{ 2 * x + y ; x in 0..5, y in 0..2; x != y};
//   ~~~~~~~~~~ 
//   ^ expression
//              ~~~~~~~~~~~~~~~~~~~~
//              ^ list of patterns
//              and iterators           ~~~~~~~
//                                      ^ [condition]
//
```

## Examples
```rust
lc!{ 2 * x ; x in 0..5 }
// -> [0,2,4,6,8]

lc!{ x.to_string() + &y.to_string() ; x in "abc".chars(), y in "de".chars()}
// -> [ad", "ae", "bd", "be", "cd", "ce"]
```

## License
MIT License

Copyright (c) 2017 Robin Guzniczak