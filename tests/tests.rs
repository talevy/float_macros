#![feature(plugin)]
#![allow(unstable)]

#[plugin] #[no_link]
extern crate float_macros;

static GOLDEN_RATIO: f32 = (1.0 + sqrt!(5.0)) / 2.0;

#[test]
fn main() {
    assert_eq!(GOLDEN_RATIO, 1.618034f32);
    assert_eq!(trunc!(2.0), 2f32);
}
