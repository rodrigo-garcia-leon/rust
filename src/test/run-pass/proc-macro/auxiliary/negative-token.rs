// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// force-host
// no-prefer-dynamic

#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::*;

#[proc_macro]
pub fn neg_one(_input: TokenStream) -> TokenStream {
    TokenTree::Literal(Literal::i32_suffixed(-1)).into()
}

#[proc_macro]
pub fn neg_one_float(_input: TokenStream) -> TokenStream {
    TokenTree::Literal(Literal::f32_suffixed(-1.0)).into()
}
