const SECRET: &[u8] = &[
    0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
    0x37, 0x38, 0x39, 0x30,
];

macro_rules! define_tests {
    ($($count:literal => $hotp_output:literal),*$(,)?) => {
        $(
            ::paste::paste! {
                #[test]
                fn [<with_ $count _expecting_ $hotp_output>]() {
                    let hotp_output = ::cerberus::hotp::hotp(SECRET, $count, ::cerberus::DEFAULT_DIGIT_NUM);
                    assert_eq!(hotp_output, $hotp_output)
                }
            }
        )*
    };
}

define_tests! {
    0 => 755224,
    1 => 287082,
    2 => 359152,
    3 => 969429,
    4 => 338314,
    5 => 254676,
    6 => 287922,
    7 => 162583,
    8 => 399871,
    9 => 520489,
}
