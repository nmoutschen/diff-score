# `diff-score`

A procedural macro crate for automatically computing difference scores between Rust data structures.

This crate provides a `#[derive(DiffScore)]` macro that generates a `diff_score(&self, other: &Self) -> f64` method, allowing you to measure structural differences between instances of the same type. It supports structs and enums, with fine-grained control over how fields and variants contribute to the score.


## Usage

```rust
use diff_score::DiffScore;

#[derive(DiffScore)]
struct MyStruct {
    num: u32,
    
    #[diff_score(weight = 2.0)]
    string: String,
}

let s1 = MyStruct { a: 10, string: "hello".to_string() };
let s2 = MyStruct { a: 10, string: "world".to_string() };

assert_eq!(s1.diff_score(&s2), 2.0);
```