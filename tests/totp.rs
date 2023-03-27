const SECRET: &[u8] = b"12345678901234567890";

macro_rules! define_tests {
    ($($time_in_secs:literal => $totp_code:literal),*$(,)?) => {
        $(
            ::paste::paste! {
                #[test]
                #[allow(clippy::zero_prefixed_literal)]
                fn [<with_ $time_in_secs _secs_expecting_ $totp_code>]() {
                    let now = ::core::time::Duration::from_secs($time_in_secs);
                    let totp_output = ::cerberus::totp::totp(SECRET, ::core::time::Duration::from_secs(0), now, 30, 8);
                    assert_eq!(totp_output, $totp_code);
                }
            }
        )*
    };
}

define_tests! {
    59 => 94287082,
    1111111109 => 07081804,
    1111111111 => 14050471,
    1234567890 => 89005924,
    2000000000 => 69279037,
    20000000000 => 65353130,
}
