use rustc_serialize::json::DecoderError;
use rustc_serialize::base64::FromBase64Error;

#[derive(Debug)]
pub enum VaultError {
    FromBase64Error(FromBase64Error),
    JsonDecoderError(DecoderError),
    VersionMatchError
}

pub type VResult<T> = Result<T, VaultError>;

impl From<DecoderError> for VaultError {
    fn from(v: DecoderError) -> VaultError {
        VaultError::JsonDecoderError(v)
    }
}

impl From<FromBase64Error> for VaultError {
    fn from(v: FromBase64Error) -> VaultError {
        VaultError::FromBase64Error(v)
    }
}
