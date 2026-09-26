use core::convert::AsRef;
use core::ops::{Deref, DerefMut};
use wdk_sys::{PCWSTR, UNICODE_STRING};

#[macro_export]
macro_rules! unicode_str {
    ($str:expr) => {{
        use wdk_sys::ntddk::RtlInitUnicodeString;
        use wdk_sys::UNICODE_STRING;

        let mut string: UNICODE_STRING = Default::default();
        unsafe { RtlInitUnicodeString(&mut string, wide!($str)) };
        string
    }};
}

#[macro_export]
macro_rules! unicode_str_from_wide_ptr {
    ($ptr:expr) => {{
        use wdk_sys::ntddk::RtlInitUnicodeString;
        use wdk_sys::UNICODE_STRING;

        let mut string: UNICODE_STRING = Default::default();
        unsafe { RtlInitUnicodeString(&mut string, $ptr) };
        string
    }};
}

#[macro_export]
macro_rules! wide {
    ($str: literal) => {{
        extern crate alloc;

        concat!($str, "\0")
            .encode_utf16()
            .collect::<alloc::vec::Vec<u16>>()
            .as_ptr()
    }};
    ($str: expr) => {{
        extern crate alloc;

        $str.encode_utf16()
            .into_iter()
            .chain(core::iter::once(0))
            .collect::<alloc::vec::Vec<u16>>()
            .as_ptr()
    }};
}

pub struct UnicodeString(UNICODE_STRING);

impl UnicodeString {
    // string must be null terminated
    pub fn from_pcwstr(wide_str: PCWSTR) -> Self {
        let raw = unicode_str_from_wide_ptr!(wide_str);
        Self { 0: raw }
    }
    pub fn from_raw(raw: UNICODE_STRING) -> Self {
        Self { 0: raw }
    }
    pub fn from_str(s: impl AsRef<str>) -> Self {
        let raw = unicode_str!(s.as_ref());
        Self { 0: raw }
    }
}

impl DerefMut for UnicodeString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for UnicodeString {
    type Target = UNICODE_STRING;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for UnicodeString {
    fn eq(&self, other: &Self) -> bool {
        if self.Length != other.Length {
            false
        } else {
            unsafe {
                core::slice::from_raw_parts(other.Buffer, other.Length.into())
                    == core::slice::from_raw_parts(self.Buffer, self.Length.into())
            }
        }
    }
}
