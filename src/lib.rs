#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(missing_docs, rust_2018_idioms, unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
//!
//! # TOTP example
//!
//! ```
//! # use cerberus::{DEFAULT_DIGIT_NUM, totp::totp};
//! # use std::time::{Duration, SystemTime, UNIX_EPOCH};
//! # let key = rand::random::<[u8; 64]>();
//! # let genesis = Duration::from_secs(0);
//! # let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
//! let totp_code = totp(&key, genesis, now, 30, DEFAULT_DIGIT_NUM);
//! println!("{totp_code}");
//! ```
//!

/// Default number of digits of the OTP code
pub const DEFAULT_DIGIT_NUM: u32 = 6;

pub mod hotp;
pub mod totp;
