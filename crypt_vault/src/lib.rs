#![feature(vec_push_all, plugin)]
#![plugin(regex_macros, clippy)]
#![deny(bad_style, unused, clippy)]

extern crate openssl;
extern crate rand;
extern crate regex;
extern crate rpassword;
extern crate rustc_serialize;

#[macro_use] mod macros;
pub mod crypto;
pub mod vault;
pub mod error;
