pub mod entry;
pub mod util;
fn main() {
    let e = entry::Entry::new();
    println!("size {:?}", e.size());
    println!("info {:?}", e.encode());
    println!("crc {:?}", e.get_crc(e.encode()));
    println!("Hello, world!");
}
