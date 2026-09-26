use kernel_types::*;

fn main() {
    let size = std::mem::size_of::<PEB>();
    let aligned_size = std::mem::size_of::<PEB_ALIGNED>();

    println!("PEB_SIZE = {size:X} PEB_ALIGNED = {aligned_size:X}");
}
