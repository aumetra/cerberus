//!
//! Implementation of the HOTP (HMAC-based one-time password) algorithm
//!

use hmac::{Hmac, Mac};
use sha1::Sha1;

/// HOTP function as defined in [RFC-4226](https://datatracker.ietf.org/doc/html/rfc4226)
#[must_use]
#[allow(clippy::missing_panics_doc)] // This should never ever panic
pub fn hotp(key: &[u8], counter: u64, digit: u32) -> u32 {
    let hmac_string = Hmac::<Sha1>::new_from_slice(key)
        .unwrap()
        .chain_update(counter.to_be_bytes())
        .finalize();

    let hmac_num = dynamic_truncate(&hmac_string.into_bytes());
    hmac_num % 10_u32.pow(digit)
}

/// Dynamic truncate function as defined in [RFC-4226, Section 5.3](https://datatracker.ietf.org/doc/html/rfc4226#section-5.3)
///
/// Panics if the slice is smaller than 20 in length
fn dynamic_truncate(bytes: &[u8]) -> u32 {
    let offset_bits = (bytes[19] & 0x0F) as usize;
    let raw_value = bytes[offset_bits..=(offset_bits + 3)].try_into().unwrap();
    u32::from_be_bytes(raw_value) & 0x7FFF_FFFF
}
