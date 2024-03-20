use super::WeChatPayError;
use aes_gcm::Error as AesGcmError;
use base64::DecodeError;
use reqwest::header::InvalidHeaderValue;
use reqwest::header::ToStrError as HttpHeaderToStrError;
use reqwest::Error as ReqwestError;
use rsa::{errors::Error as RsaError, pkcs8::Error as Pkcs8DecodeError};
use serde_json::Error as SerdeError;
use std::{io::Error as IOError, str::Utf8Error, time::SystemTimeError};
use url::ParseError;

impl From<AesGcmError> for WeChatPayError {
  fn from(_: AesGcmError) -> Self {
    Self::CryptoError("AEAD_AES_256_GCM error".to_string())
  }
}

impl From<DecodeError> for WeChatPayError {
  fn from(_: DecodeError) -> Self {
    Self::Unknown("Base64 Decode Error".to_string())
  }
}

impl From<InvalidHeaderValue> for WeChatPayError {
  fn from(err: InvalidHeaderValue) -> Self {
    Self::Unknown(err.to_string())
  }
}

impl From<HttpHeaderToStrError> for WeChatPayError {
  fn from(err: HttpHeaderToStrError) -> Self {
    Self::Unknown(err.to_string())
  }
}

impl From<ReqwestError> for WeChatPayError {
  fn from(err: ReqwestError) -> Self {
    Self::NetworkError(err)
  }
}

impl From<SerdeError> for WeChatPayError {
  fn from(err: SerdeError) -> Self {
    Self::Unknown(err.to_string())
  }
}

impl From<RsaError> for WeChatPayError {
  fn from(err: RsaError) -> Self {
    Self::CryptoError(err.to_string())
  }
}

impl From<Pkcs8DecodeError> for WeChatPayError {
  fn from(err: Pkcs8DecodeError) -> Self {
    Self::CryptoError(err.to_string())
  }
}

impl From<IOError> for WeChatPayError {
  fn from(err: IOError) -> Self {
    Self::InternalServerError(err.to_string())
  }
}

impl From<Utf8Error> for WeChatPayError {
  fn from(_: Utf8Error) -> Self {
    Self::Unknown("UTF-8 Decode Error".to_string())
  }
}

impl From<SystemTimeError> for WeChatPayError {
  fn from(err: SystemTimeError) -> Self {
    Self::InternalServerError(err.to_string())
  }
}

impl From<ParseError> for WeChatPayError {
  fn from(err: ParseError) -> Self {
    Self::Unknown(err.to_string())
  }
}
