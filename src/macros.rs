#![allow(missing_docs)]

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let mut buf = String::new();
        // Write the formatted arguments into the buffer
        core::fmt::write(&mut buf, core::format_args!($($arg)*)).unwrap();
        buf
    }};
}
