#![cfg_attr(any(const_unstable), feature(const_fn, const_vec_new))]
#![deny(warnings)]
#![allow(dead_code)]
#![cfg(test)]

extern crate const_fn;

use const_fn::const_fn;

#[test]
fn test_variables() {
    assert!(const_min("variables") == "variables");
    assert_eq!(const_let("variables"), "variables");
    assert_eq!(const_vec_new::<u8>(), Vec::new());
}

// min_const_fn (rust 1.31+)

#[const_fn(min_const_fn)]
fn const_min<T>(x: T) -> T {
    x
}

#[cfg(min_const_fn)]
const CONST_MIN: &str = const_min("min_const_fn");

#[cfg(min_const_fn)]
#[test]
fn test_const_min() {
    assert!(CONST_MIN == "min_const_fn");
    assert_eq!(const_let("min_const_fn"), "min_const_fn");
    assert_eq!(const_vec_new::<u8>(), Vec::new());
}

// const_let (rust 1.33+)

#[const_fn(const_let)]
fn const_let<T>(x: T) -> T {
    let y = const_min(x);
    y
}

#[cfg(const_let)]
const CONST_LET: &str = const_let("const_let");

#[cfg(const_let)]
#[test]
fn test_const_let() {
    assert!(CONST_LET == "const_let");
    assert_eq!(const_vec_new::<u8>(), Vec::new());
}

// const_fn + const_vec_new (rust nightly)

#[const_fn(const_unstable)]
fn const_vec_new<T>() -> Vec<T> {
    let vec = Vec::new();
    vec
}

#[cfg(const_unstable)]
const CONST_UNSTABLE: Vec<u8> = const_vec_new();

#[cfg(const_unstable)]
#[test]
fn test_const_unstable() {
    assert_eq!(CONST_UNSTABLE, Vec::new());
}
