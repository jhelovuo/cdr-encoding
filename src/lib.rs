use core::ops::Mul;
use core::ops::Add;

pub use cdr_encoding_size_derive::*;

/// This type is used to count maximum size of a Key when serialized
/// according to CDR but without field alignment.
/// The purpose is to find out if the (key) type always fits into 16 bytes or not.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd,)]
pub enum CdrEncodingMaxSize {
  Bytes(usize),
  Unbounded,
}

impl Add for CdrEncodingMaxSize {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
      use CdrEncodingMaxSize::*;
      match (self,other) {
        (Bytes(s),Bytes(o)) => Bytes(s+o),
        (Unbounded,_) => Unbounded,
        (_,Unbounded) => Unbounded,
      }
    }
}


impl Mul<usize> for CdrEncodingMaxSize {
  type Output = Self;

  fn mul(self, rhs: usize) -> Self::Output {
    use CdrEncodingMaxSize::*;
    match self {
      Unbounded => Unbounded,
      Bytes(b) => Bytes(b*rhs),
    }
  }
}

// -------------------------------------------
// -------------------------------------------

pub trait CdrEncodingSize
{
  fn cdr_encoding_max_size() -> CdrEncodingMaxSize;
}  

// -------------------------------------------
// -------------------------------------------


impl CdrEncodingSize for () {
  fn cdr_encoding_max_size() -> CdrEncodingMaxSize
  { CdrEncodingMaxSize::Bytes(0) }
}

macro_rules! prim_cdr_encoding_size {
  ($t:ty) => {
    impl CdrEncodingSize for $t {
      #[inline]
      fn cdr_encoding_max_size() -> CdrEncodingMaxSize
      { CdrEncodingMaxSize::Bytes( std::mem::size_of::<$t>() ) }
    }
  };
}

prim_cdr_encoding_size!(u8);
prim_cdr_encoding_size!(u16);
prim_cdr_encoding_size!(u32);
prim_cdr_encoding_size!(u64);
prim_cdr_encoding_size!(u128);

prim_cdr_encoding_size!(i8);
prim_cdr_encoding_size!(i16);
prim_cdr_encoding_size!(i32);
prim_cdr_encoding_size!(i64);
prim_cdr_encoding_size!(i128);

prim_cdr_encoding_size!(char);
//prim_cdr_encoding_size!(usize); // size is platform-dependent, do not use in serializable data
//prim_cdr_encoding_size!(isize);

impl CdrEncodingSize for bool {
  fn cdr_encoding_max_size() -> CdrEncodingMaxSize
  { CdrEncodingMaxSize::Bytes(1) }
}


impl<T: CdrEncodingSize> CdrEncodingSize for Vec<T> {
  fn cdr_encoding_max_size() -> CdrEncodingMaxSize
  { CdrEncodingMaxSize::Unbounded }
}

impl CdrEncodingSize for String {
  fn cdr_encoding_max_size() -> CdrEncodingMaxSize
  { CdrEncodingMaxSize::Unbounded }
}

impl<T: CdrEncodingSize> CdrEncodingSize for Box<T> {
  fn cdr_encoding_max_size() -> CdrEncodingMaxSize
  { T::cdr_encoding_max_size() }
}

impl<T: CdrEncodingSize, const N: usize> CdrEncodingSize for [T; N] {
  fn cdr_encoding_max_size() -> CdrEncodingMaxSize
  { T::cdr_encoding_max_size() * N }
}

