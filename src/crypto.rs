use openssl::crypto::pkcs5::pbkdf2_hmac_sha1;
use openssl::crypto::symm;
use rand::{Rng, OsRng};
use rpassword;

use std::io::{stdout, Write};
use std::str;

pub type Iv = [u8; 16];
pub type Salt = [u8; 16];
pub fn encrypt(key: &[u8], string: &str) -> (Iv, Vec<u8>) {
    let mut iv = [0; 16];
    let mut f = OsRng::new().ok().expect("Unable to use OS Rng. Can't save");
    f.fill_bytes(&mut iv);

    let data: Vec<u8> = string.bytes().collect();

    let crypter = symm::Crypter::new(symm::Type::AES_256_CBC);
    crypter.pad(true);
    crypter.init(symm::Mode::Encrypt, key, &iv);
    let mut final_result = Vec::new();
    final_result.push_all(&crypter.update(&data));
    final_result.push_all(&crypter.finalize());

    (iv, final_result)
}

pub fn decrypt(key: &[u8], iv: &[u8], data: &[u8]) -> String {
    let crypter = symm::Crypter::new(symm::Type::AES_256_CBC);
    crypter.pad(true);
    crypter.init(symm::Mode::Decrypt, key, iv);
    let mut final_result = Vec::new();
    final_result.push_all(&crypter.update(data));
    final_result.push_all(&crypter.finalize());

    str::from_utf8(&final_result).ok().unwrap_or("").to_owned()
}

pub fn derive_key(p: &str, salt: &[u8]) -> Vec<u8> {
    pbkdf2_hmac_sha1(p, salt, 1024, 32)
}

pub fn getpass() -> String {
    loop {
        print!("Enter master password: ");
        stdout().flush().ok().expect("Unable to flush stdout");
        if let Ok(p) = rpassword::read_password() {
            return p;
        }
    }
}

pub fn gen_bytes() -> Vec<u8> {
    let mut salt = [0; 16];
    let mut f = OsRng::new().ok().expect("Unable to use OS Rng. Can't save");
    f.fill_bytes(&mut salt);

    let mut ret = Vec::new();
    ret.push_all(&salt);
    ret
}
