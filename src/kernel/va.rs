use super::*;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Va(pub usize);

impl From<Va> for PVOID {
    fn from(value: Va) -> Self {
        value.0 as PVOID
    }
}
