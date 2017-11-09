# rs-list-comp - List Comprehension for Rust

Provides basic list comprehension for Rust as a macro. 

## Usage
```rust
let I = lc!{ 2 * x + y ; x in 0..5, y in 0..2; x != y};
            ~~~~~~~~~~ expression mapped to every value
                         ~~~~~~~~~~~~~~~~~~~~ list of iterators
                                              ~~~~~~~ condition

```

## Example
```rust
lc!{ 2 * x ; x in 0..5 } // [0,2,4,6,8]
lc!{ x.to_string() + &y.to_string() ; x in "abc".chars(), y in "de".chars()} // [ad", "ae", "bd", "be", "cd", "ce"]
```