use windows_types::kernel::eprocess::{EPROCESS, KPROCESS};
use windows_types::kernel::peb::{PEB, RTL_USER_PROCESS_PARAMETERS};

fn main() {
    let size = std::mem::size_of::<PEB>();
    println!("{size:X?}");
    let size = std::mem::size_of::<EPROCESS>();
    println!("{size:X?}");
    let size = std::mem::size_of::<KPROCESS>();
    println!("{size:X?}");
    let size = std::mem::size_of::<RTL_USER_PROCESS_PARAMETERS>();
    println!("{size:X?}");
}
