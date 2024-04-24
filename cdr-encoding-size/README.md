This crate specifies the `CdrEncodingSize` trait. It is a helper to the [RustDDS](https://github.com/jhelovuo/RustDDS) library.

The trait is in a separate crate, because it has a [derive-macro](https://github.com/jhelovuo/cdr-encoding/tree/master/cdr-encoding-size-derive),
and (the Rust Programming Language, Section "19.5 Macros")[https://doc.rust-lang.org/book/ch19-06-macros.html] advises that "At the time of this writing, procedural macros need to be in their own crate. Eventually, this restriction might be lifted."

The RTPS specification version 2.5 Section "9.6.4.8 KeyHash (PID_KEY_HASH)" defines an algorithm for computing a 16-byte hash of a DDS Key of a data sample.

The algorithm summary:

* Define a "holder" type corresponding to the data sample type.
* The holder type consists of all the fields that form the Key of the data sample type.
* The holder type is serialized according to PLAIN_CDR2 Big Endian rules.
* If the holder type has maximum serialized size of at most 16 bytes, the serialized representation produced in the previous step, padded with zero bytes to be exactly 16 bytes, is the "key hash".
* If the holder type has maximum serialized size of more than 16 bytes, then the MD5 hash of the serialized representation above is computed, and this is the "key hash"
* This always results in a "key hash" of exactly 16 bytes.

This trait is an automated mechanism for deciding if the 16-byte limit is exceeded, i.e. if MD5 hashing should be used or not.

TODO

* Derive macro supports primitive types and `struct`s, but not `enum` types. It will fail if a Key type is, or contains, an `enum`.