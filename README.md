# float_macros

Compile time function evaluation for some Float trait functions

Currently only supports methods with the signature `fn <function_name>(self) -> self`
where `self` can be of type `f32` or `f64`, but `f32` is returned for both (for now).

## Motivation

To be able to declare static mathematical variables as closed-form expressions.

## Requirements
Include the following in your `Cargo.toml`:

```toml
[dependencies.float_macros]
git = "https://github.com/talevy/float_macros"
```

In your `lib.rs`:

```rust
#![feature(plugin)]                                                                                                                                                       
#![allow(unstable)]                                                                                                                                                       

#[plugin] #[no_link]                                                                                                                                                      
extern crate float_macros;
```

## Usage

```rust
#![feature(plugin)]
#![allow(unstable)]

#[plugin] #[no_link]
extern crate float_macros;

static GOLDEN_RATIO: f32 = (1.0 + sqrt!(5.0)) / 2.0;

fn main() {
  println!("{}", GOLDEN_RATIO);  
}
```
