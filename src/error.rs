use rustc_serialize::{json, base64};
#[derive(Debug)]
pub enum PassmanError {
    FromBase64Error(base64::FromBase64Error),
    JsonDecoderError(json::DecoderError),
    VersionMatchError
}

pub type PMResult<T> = Result<T, PassmanError>;

impl From<json::DecoderError> for PassmanError {
    fn from(v: json::DecoderError) -> PassmanError {
        PassmanError::JsonDecoderError(v)
    }
}

impl From<base64::FromBase64Error> for PassmanError {
    fn from(v: base64::FromBase64Error) -> PassmanError {
        PassmanError::FromBase64Error(v)
    }
}
