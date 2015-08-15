#![feature(plugin)]
#![deny(bad_style, unused)]

extern crate rustc_serialize;
extern crate crypto_vault;
extern crate rpassword;

use crypto_vault::{RawVault, Vault, VaultError};
use rustc_serialize::{Decoder, Encoder};

use std::io::{Read, Write};
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::fmt::{Display, Formatter, Error};

pub fn getpass() -> String {
    loop {
        print!("Enter master password: ");
        stdout().flush().ok().expect("Unable to flush stdout");
        if let Ok(pass) = rpassword::read_password() {
            return pass
        }
    }
}

use CommandMode::*;
#[derive(Clone)]
enum CommandMode {
    Oneshot,
    REPL
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
enum PassmanEntry {
    V0(V0Entry),
    V1(V1Entry)
}

impl From<V0Entry> for V1Entry {
    fn from(e: V0Entry) -> V1Entry {
        match e {
            V0Entry::Password { name, url } => V1Entry::PW(Password { name: name, url: url, password: "hi".to_owned() }),
        }
    }
}

impl PassmanEntry {
    fn upgrade(self) -> PassmanEntry {
        use PassmanEntry::*;
        match self {
            V0(entry) => V1(V1Entry::from(entry)),
            other => other
        }
    }
}

impl Display for PassmanEntry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            PassmanEntry::V0(ref entry) => write!(f, "{}", entry),
            PassmanEntry::V1(ref entry) => write!(f, "{}", entry)
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
struct Password { name: String, url: Option<String>, password: String }

#[derive(RustcEncodable, RustcDecodable, Debug)]
enum V1Entry {
    PW(Password)
}

impl Display for V1Entry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use V1Entry::*;
        match *self {
            PW(Password { ref name, url: Some(ref url), ref password }) => {
                write!(f, "{}: {}   ~~ {}", name, url, password)
            },
            PW(Password { ref name, ref password, .. }) => {
                write!(f, "{}   ~~ {}", name, password)
            }
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
enum V0Entry {
    Password { name: String, url: Option<String> }
}

impl Display for V0Entry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use V0Entry::*;
        match *self {
            Password { ref name, ref url, .. } => {
                if let &Some(ref url) = url {
                    write!(f, "{} {}", name, url)
                } else {
                    write!(f, "{}", name)
                }
            }
        }
    }
}

fn get_vault() -> Vault<PassmanEntry> {
    let mut file = env::home_dir().unwrap();
    file.push(".passman");
    let mut vault: Vault<PassmanEntry> = fs::File::open(file).ok().and_then(|mut file| {
        let mut data = String::new();
        file.read_to_string(&mut data).ok().expect("Nothing in data file!");
        let raw: RawVault = data.parse();
        loop {
            match raw.decrypt(&getpass()) {
            }
        }
        /*return loop_until_some! {
            match data.parse() {
                Err(VaultError::BadPasswordError) => {
                    println!("Bad password");
                    None
                },
                d => {
                    println!("{:?}", d);
                    Some(d.ok())
                }
            }
        }*/
    }).unwrap_or_default();

    vault.objects = vault.objects.into_iter().map(|obj| obj.upgrade()).collect();

    vault
}

fn usage(mode: CommandMode) {
    match mode {
        Oneshot => println!("PASSMAN"),
        REPL => println!("Commands: list")
    }
}

fn run_command(mode: CommandMode, vault: &Vault<PassmanEntry>, args: &[String]) {
    let mut a = args.iter();
    match a.next().unwrap_or(&"".to_owned()).as_ref() {
        "exit" | "quit" => ::std::process::exit(0),
        "list" => for entry in vault.objects.iter() {
            println!("{}", entry);
        },
        _ => usage(mode)
    }
}

fn prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut vault = get_vault();
    println!("{:?}", vault);
    if !args.is_empty() {
        run_command(Oneshot, &mut vault, &args);
    } else {
        let stdin = io::stdin();
        prompt();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                run_command(REPL, &mut vault, &line.split(" ").map(|s| s.to_owned()).collect::<Vec<String>>());
            }
            prompt();
        }
    }
}
