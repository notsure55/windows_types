struct PPeb(*mut kernel::PEB);

impl PPeb {
    pub fn from_raw(raw: *mut kernel::PEB) -> Self {
        Self { 0: raw }
    }
}

impl Deref for PPeb {
    type Target = PEB;
    pub fn deref(&self) -> &Self::Target {
        match unsafe { self.0.as_ref() } {
            Some(targ) => targ,
            None => panic!("reference was nullpointer in {:#X?}", self),
        }
    }
}

impl DerefMut for PPeb {
    pub fn deref_mut(&self) -> &mut Self::Target {
        match unsafe { self.0.as_mut() } {
            Some(targ) => targ,
            None => panic!("mut reference was nullpointer in {:#X?}", self),
        }
    }
}

// ideal api
fn main() {
    let peb = PPeb::from_raw(core::ptr::null_mut());

    let image_path = peb.ProcessParameters.image_path_name;
}
