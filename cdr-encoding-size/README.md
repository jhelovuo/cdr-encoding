This crate specifies the CdrEncodingSize trait. It is a helper to the [RustDDS](https://github.com/jhelovuo/RustDDS) library.

The trait is in a separate crate, because it has a [derive-macro](https://github.com/jhelovuo/cdr-encoding-size-derive).

The RTPS specification version 2.5 Section "9.6.4.8 KeyHash (PID_KEY_HASH)" defines an algorithm for computing a 16-byte hash of a DDS Key of a data sample.

The algorithm summary:

* Define a "holder" type corresponding to the data sample type.
* The holder type consists of all the fields that form the Key of the data sample type.
* The holder type is serialized according to PLAIN_CDR2 Big Endian.
* If the holder type has maximum serialized size of at most 16 bytes, the serialized representation is the key hash, padded with zero bytes to 16 bytes.
* If the holder type has maximum serialized size of more then 16 bytes, key hash is the MD5 hash of the serialized representation.
* This always results in a key hash of exactly 16 bytes.

This trait is a means of deciding if the 16-byte limit is exceede, i.e. if MD5 shold be used or not.