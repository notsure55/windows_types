extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
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
    ($str: literal) => {
        concat!($str, "\0")
            .encode_utf16()
            .collect::<alloc::vec::Vec<u16>>()
            .as_ptr()
    };
    ($str: expr) => {
        $str.encode_utf16()
            .into_iter()
            .chain(core::iter::once(0))
            .collect::<alloc::vec::Vec<u16>>()
            .as_ptr()
    };
}

#[derive(Debug, Clone)]
#[repr(C)]
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
    pub fn contains(&self, other: &UnicodeString) -> bool {
        let o = other.to_string();
        let s = self.to_string();

        s.contains(&o)
    }
    pub fn as_slice(&self) -> &[u16] {
        unsafe { core::slice::from_raw_parts(self.Buffer, self.len()) }
    }
    pub fn len(&self) -> usize {
        usize::from(self.Length) / 2
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
            unsafe { other.as_slice() == self.as_slice() }
        }
    }
}

impl ToString for UnicodeString {
    fn to_string(&self) -> String {
        let slice = self.as_slice();
        String::from_utf16_lossy(slice)
    }
}
