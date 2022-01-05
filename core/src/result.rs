use std::{io::Error as IoError, result, string};

#[derive(Debug, Clone)]
pub enum SourceMapErrorType {
  ParcelSourceMap,
  SerdeSerialization,

  UTF8,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
  pub error_type: SourceMapErrorType,
  pub reason: Option<String>,
}

impl Error {
  pub fn new(error_type: SourceMapErrorType) -> Self {
    Self {
      error_type,
      reason: None,
    }
  }

  pub fn new_with_reason(error_type: SourceMapErrorType, reason: &str) -> Self {
    Self {
      error_type,
      reason: Some(String::from(reason)),
    }
  }
}

impl From<parcel_sourcemap::SourceMapError> for Error {
  #[inline]
  fn from(err: parcel_sourcemap::SourceMapError) -> Self {
    match err.reason {
      Some(r) => Error::new_with_reason(SourceMapErrorType::ParcelSourceMap, r.as_str()),
      None => Error::new(SourceMapErrorType::ParcelSourceMap),
    }
  }
}

impl From<string::FromUtf8Error> for Error {
  #[inline]
  fn from(_: string::FromUtf8Error) -> Self {
    Error::new(SourceMapErrorType::UTF8)
  }
}

impl From<serde_json::Error> for Error {
  #[inline]
  fn from(err: serde_json::Error) -> Self {
    let io_error: IoError = err.into();

    Error::new_with_reason(
      SourceMapErrorType::SerdeSerialization,
      io_error.to_string().as_str(),
    )
  }
}
