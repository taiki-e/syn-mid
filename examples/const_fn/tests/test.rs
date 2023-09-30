// SPDX-License-Identifier: Apache-2.0 OR MIT

use const_fn::const_fn;

#[const_fn(any())]
pub const fn a() {}
#[const_fn(not(any()))]
const fn b() {}

pub static _B: () = b();
