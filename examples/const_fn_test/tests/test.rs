#![cfg_attr(nightly, feature(const_fn, const_vec_new))]
#![deny(warnings)]
#![allow(dead_code)]
#![cfg(test)]

extern crate const_fn;

use const_fn::const_fn;

#[const_fn(nightly)]
fn const_vec_new<T>() -> Vec<T> {
    let vec = Vec::new();
    vec
}

#[test]
fn test_stable() {
    assert_eq!(const_vec_new::<u8>(), Vec::new());
}

#[cfg(nightly)]
const CONST_UNSTABLE: Vec<u8> = const_vec_new();

#[cfg(nightly)]
#[test]
fn test_unstable() {
    assert_eq!(CONST_UNSTABLE, Vec::new());
}
