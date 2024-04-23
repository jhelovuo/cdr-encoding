use std::fmt::Display;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

/// Several things can go wrong during CDR (de)serialization
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Wrapper for error string
  #[error("{0}")]
  Message(String),

  /// Wrapper for `std::io::Error`
  #[error("io::Error: {0}")]
  Io(#[from] std::io::Error),

  /// CDR is not self-describing format, cannot deserialize \'Any\' type.
  /// This is an indication of trying something that is not possible with CDR.
  #[error("CDR is not self-describing format, cannot deserialize \'Any\' type: {0}")]
  NotSelfDescribingFormat(String),

  /// Serialization must know sequence length before serialization.
  /// This is an indication of trying something that is not possible with CDR.
  #[error("CDR serialization requires sequence length to be specified at the start.")]
  SequenceLengthUnknown,

  /// Unexpected end of input
  #[error("unexpected end of input")]
  Eof,

  /// Bad encoding of Boolean value
  #[error("Expected 0 or 1 as Boolean, got: {0}")]
  BadBoolean(u8),

  /// Bad Unicode codepoint
  #[error("Bad Unicode character code: {0}")]
  BadChar(u32), // invalid Unicode codepoint

  /// Bad discriminant (variant tag) in `Option`
  #[error("Option value must have discriminant 0 or 1, read: {0}")]
  BadOption(u32), // Option variant tag (discriminant) is not 0 or 1

  /// String was not valid UTF-8
  #[error("UTF-8 error: {0}")]
  BadUTF8(std::str::Utf8Error),
}

impl ser::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Self::Message(msg.to_string())
  }
}

impl de::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Self::Message(msg.to_string())
  }
}
