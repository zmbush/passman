#![feature(vec_push_all, plugin)]
#![plugin(regex_macros, clippy)]
#![deny(
    bad_style,
    unused,
    clippy
)]

extern crate openssl;
extern crate rand;
extern crate regex;
extern crate rpassword;
extern crate rustc_serialize;
extern crate toml;

mod vault;
mod crypto;
mod error;

use error::PassmanError;
use rustc_serialize::{Decoder, Encoder};
use vault::Vault;

use std::io::{Read, Write};
use std::env;
use std::fs;

#[derive(RustcEncodable, RustcDecodable, Debug)]
struct Password;

fn get_vault() -> Vault<Password> {
    let mut file = env::home_dir().unwrap();
    file.push(".passman");
    fs::File::open(file).ok().and_then(|mut file| {
        let mut data = String::new();
        file.read_to_string(&mut data).ok().expect("Nothing in data file!");
        loop {
            let ret = data.parse();
            if let Err(PassmanError::JsonDecoderError(_)) = ret {
                println!("Bad password");
            } else {
                return ret.ok();
            }
        }
    }).unwrap_or_default()
}

fn main() {
    let vault = get_vault();
    println!("read vault: {:?}", vault);
    let mut file = env::home_dir().unwrap();
    file.push(".passman");
    fs::File::create(file).map(|mut file| write!(file, "{}", vault.to_string()).unwrap()).unwrap();
}
