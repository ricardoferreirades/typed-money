pub use crate::format;

pub type String = arrayvec::ArrayString<102>;

pub trait ToArrayString {
    fn to_string(&self) -> String;
}

impl<T: core::fmt::Display> ToArrayString for T {
    fn to_string(&self) -> String {
        format!("{self}")
    }
}
