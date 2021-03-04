use byteorder::{LittleEndian, WriteBytesExt};
use std::mem;

pub fn set_buf_u64(mut buf: Vec<u8>, offset: usize, val: u64) -> Vec<u8> {
	let mut tmp = [0u8; mem::size_of::<u64>()];
	tmp.as_mut()
	.write_u64::<LittleEndian>(val)
	.expect("Unable to write");
	for (k, v) in tmp.iter().enumerate() {
		buf[offset+k] = *v;
	} 
	return buf
}

pub fn set_buf_u32(mut buf: Vec<u8>, offset: usize, val: u32) -> Vec<u8> {
	let mut tmp = [0u8; mem::size_of::<u32>()];
	tmp.as_mut()
	.write_u32::<LittleEndian>(val)
	.expect("Unable to write");
	for (k, v) in tmp.iter().enumerate() {
		buf[offset+k] = *v;
	} 
	return buf
}

pub fn set_buf_u16(mut buf: Vec<u8>, offset: usize, val: u16) -> Vec<u8> {
	let mut tmp = [0u8; mem::size_of::<u16>()];
	tmp.as_mut()
	.write_u16::<LittleEndian>(val)
	.expect("Unable to write");
	for (k, v) in tmp.iter().enumerate() {
		buf[offset+k] = *v;
	} 
	return buf
}
