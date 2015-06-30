use rustc_serialize::{json, Decodable, Encodable};
use rustc_serialize::base64::{self, ToBase64, FromBase64};
use std::fmt::Debug;
use std::str::FromStr;

use error::{VaultError, VResult};
use crypto::{encrypt, decrypt, gen_bytes, getpass, derive_key};

#[derive(Debug)]
pub struct Vault<T: Decodable + Encodable + Debug> {
    pub version: String,
    master_key: Option<Vec<u8>>,
    salt: Option<Vec<u8>>,
    pub objects: Vec<T>
}

impl<T: Decodable + Encodable + Debug> Default for Vault<T> {
    fn default() -> Vault<T> {
        Vault {
            version: "1.0".to_owned(),
            master_key: None,
            salt: None,
            objects: Vec::new()
        }
    }
}

impl<T: Decodable + Encodable + Debug> ToString for Vault<T> {
    fn to_string(&self) -> String {
        let salt = self.salt.clone().unwrap_or_else(gen_bytes);
        let master_key = self.master_key.clone().unwrap_or_else(|| derive_key(&getpass(), &salt));
        let data = json::as_json(&self.objects);
        let (iv, data) = encrypt(&master_key, &data.to_string());
        format!(r"Vault v{} https://github.com/zmbush/passman
{}
{}
{}",
            self.version,
            salt.to_base64(base64::STANDARD),
            iv.to_base64(base64::STANDARD),
            data.to_base64(base64::Config { newline: base64::Newline::LF, .. base64::MIME })
        )
    }
}

impl<T: Decodable + Encodable + Debug> FromStr for Vault<T> {
    type Err = VaultError;

    fn from_str(s: &str) -> VResult<Vault<T>> {
        let re = regex!(r"v([.\d]+)");

        let mut lines = s.lines_any();
        let version = match re.captures(lines.next().unwrap_or("")) {
            Some(cap) => match cap.at(1) {
                Some(mat) => mat,
                None => return Err(VaultError::VersionMatchError)
            },
            None => return Err(VaultError::VersionMatchError)
        };

        let salt = try!(lines.next().unwrap_or("").from_base64());
        let master_key = derive_key(&getpass(), &salt);

        let iv = try!(lines.next().unwrap_or("").from_base64());
        let data = try!(lines.fold("".to_owned(), |acc, item| format!("{}{}", acc, item)).from_base64());

        Ok(Vault {
            version: version.to_owned(),
            salt: Some(salt),
            objects: try!(json::decode(&decrypt(&master_key, &iv, &data))),
            master_key: Some(master_key),
        })
    }
}
