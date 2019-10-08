//! Oasis runtime.
#[cfg(feature = "test")]
extern crate byteorder;
#[cfg(feature = "test")]
extern crate elastic_array;
#[cfg(feature = "test")]
extern crate ethkey;
#[cfg(feature = "test")]
#[macro_use]
extern crate serde_json;

pub mod block;
pub mod methods;

#[cfg(feature = "test")]
pub mod test;

// Include key manager enclave hash.
include!(concat!(env!("OUT_DIR"), "/km_enclave_hash.rs"));
