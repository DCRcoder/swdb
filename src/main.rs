pub mod entry;
pub mod util;
pub mod file_manager;
pub mod error;
fn main() {
    let e = entry::Entry::new();
    println!("size {:?}", e.size());
    println!("info {:?}", e.encode());
    println!("crc {:?}", e.get_crc(e.encode()));
    println!("Hello, world!");
}
