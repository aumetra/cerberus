//!
//! Implementation of the TOTP (time-based one-time password) algorithm
//!
//! This is an extension of the HOTP algorithm that is also implemented in this crate
//!

use crate::hotp::hotp;
use core::time::Duration;

/// Implementation of the TOTP function
///
/// Parameters
///
/// - `key`: The key shared between the verifier and the TOTP client
/// - `genesis`: The timestamp at which the first token was created, expressed as the duration since the unix epoch
/// - `now`: The current timestamp expressed as the duration since the unix epoch
/// - `timestep`: Length of a timestep in seconds
/// - `digit`: How many digits the code is supposed to have
#[must_use]
pub fn totp(key: &[u8], genesis: Duration, now: Duration, timestep: u64, digit: u32) -> u32 {
    let counter = (now - genesis).as_secs() / timestep;
    hotp(key, counter, digit)
}
