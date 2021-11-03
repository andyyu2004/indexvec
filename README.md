# Newtype Index

A simplified copy of [rustc's index crate](https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_index).

## Examples

```rust
newtype_index! {
    pub struct MyIndex;
}

// Shorthand for the above
newtype_index!(pub MyIndex);

// Full example of all features
newtype_index! {
    #[derive(SomeTrait)]
    pub struct MyIndex {
        pub const A = 5;
        pub const B = 999;
    }
}
```

Serde can be enabled with the `serde` feature.