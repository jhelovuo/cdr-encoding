//! OMG Common Data Representation (CDR) serialization with Serde
//! See [Wikipedia](https://en.wikipedia.org/wiki/Common_Data_Representation) or 
//! [specification in Section "9.3 CDR Transfer Syntax"](https://www.omg.org/spec/CORBA/3.4/Interoperability/PDF).
//!
//! [Full XTYPES specification](https://www.omg.org/spec/DDS-XTypes/1.2/PDF), which covers a lot more.
//! This implemention is only for "plain" CDR.
//!
//! # Example
//!
//! ```
//!  use cdr_encoding::*;
//!  use serde::{Serialize, Deserialize};
//!  use byteorder::LittleEndian;
//!
//!  // This example is originally from https://www.omg.org/spec/DDSI-RTPS/2.3/PDF
//!  // 10.7 Example for User-defined Topic Data
//!  #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
//!  struct ShapeType {
//!    color: String,
//!    x: i32,
//!    y: i32,
//!    size: i32,
//!  }
//!
//!  let message = ShapeType {
//!    color: "BLUE".to_string(),
//!    x: 34,
//!    y: 100,
//!    size: 24,
//!  };
//!
//!  let expected_serialized_result: Vec<u8> = vec![
//!    0x05, 0x00, 0x00, 0x00, 0x42, 0x4c, 0x55, 0x45, 0x00, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00,
//!    0x00, 0x64, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00,
//!  ];
//!
//!  let serialized = to_vec::<ShapeType, LittleEndian>(&message).unwrap();
//!  assert_eq!(serialized, expected_serialized_result);
//!
//!  let (deserialized_message, _consumed_byte_count) 
//!    = from_bytes::<ShapeType, LittleEndian>(&serialized).unwrap();
//!  assert_eq!(deserialized_message, message);
//! ```



mod cdr_deserializer;
mod cdr_serializer;
mod error;

pub use cdr_deserializer::{from_bytes, CdrDeserializer};
pub use cdr_serializer::{to_vec, to_vec_with_size_hint, to_writer, CdrSerializer};
pub use error::{Error, Result};
