#![cfg_attr(nightly, feature(const_fn))]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use const_fn::const_fn;

struct A<T> {
    x: T,
}

impl<T: IntoIterator> A<T> {
    #[const_fn(nightly)]
    const fn new(x: T) -> Self {
        Self { x }
    }
}

#[test]
fn test_stable() {
    assert_eq!(A::new(Vec::<u8>::new()).x, Vec::new());
}

#[cfg(nightly)]
const CONST_UNSTABLE: A<Vec<u8>> = A::new(Vec::new());

#[cfg(nightly)]
#[test]
fn test_unstable() {
    assert_eq!(CONST_UNSTABLE.x, Vec::new());
}
