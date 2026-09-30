use super::*;

#[repr(C)]
pub struct Va(pub usize);

impl From<Va> for PVOID {
    fn from(value: Va) -> Self {
        value.0 as PVOID
    }
}
