#![feature(vec_push_all, plugin)]
#![plugin(regex_macros, clippy)]
#![deny(bad_style, unused, clippy)]

extern crate openssl;
extern crate rand;
extern crate regex;
extern crate rpassword;
extern crate rustc_serialize;

#[macro_use] mod macros;
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
struct Password {
    name: String,
    url: Option<String>
}

fn get_vault() -> Vault<Password> {
    let mut file = env::home_dir().unwrap();
    file.push(".passman");
    fs::File::open(file).ok().and_then(|mut file| {
        let mut data = String::new();
        file.read_to_string(&mut data).ok().expect("Nothing in data file!");
        return loop_until_some! {
            match data.parse() {
                Err(PassmanError::JsonDecoderError(_)) => {
                    println!("Bad password");
                    None
                },
                d => Some(d.ok())
            }
        }
    }).unwrap_or_default()
}

fn main() {
    let mut vault = get_vault();
    vault.objects.push(Password {
        name: "Walter Of Archibald".to_owned(),
        url: Some("hi".to_owned())
    });
    println!("read vault: {:?}", vault);
    let mut file = env::home_dir().unwrap();
    file.push(".passman");
    fs::File::create(file).map(|mut file| write!(file, "{}", vault.to_string()).unwrap()).unwrap();
}
