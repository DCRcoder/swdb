use std::vec;
use crate::util::{set_buf_u32, set_buf_u64, set_buf_u16};

/// DATA_ENTRY_HEADER_SIZE returns the entry header size
pub static  DATA_ENTRY_HEADER_SIZE: i64 = 42;
/// Entry 
#[derive(Debug, Clone)]
pub struct Entry {
    pub key: ::std::vec::Vec<u8>, 
    pub value: ::std::vec::Vec<u8>,
    pub position: u64,
    pub meta: Meta,
}


impl Entry {
    pub fn new() -> Entry {
        return Entry{
            key: vec![],
            value: vec![],
            position: 0,
            meta: Meta::new(),
        };
    }

    pub fn size(&self) -> i64 {
        return DATA_ENTRY_HEADER_SIZE + (self.meta.key_size + self.meta.value_size + self.meta.bucket_size) as i64
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = vec![0;self.size() as usize];
        buf = self.set_entry_header_buf(buf);
        buf.extend_from_slice(self.meta.bucket.clone().as_slice());
        buf.extend_from_slice(self.key.clone().as_slice());
        buf.extend_from_slice(self.value.clone().as_slice());
        let mut digest = crc32fast::Hasher::new();
        digest.update(&buf[4..]);
        buf = set_buf_u32(buf,0, digest.finalize());
        return buf;
    }

    pub fn set_entry_header_buf(&self, mut buf: Vec<u8>) -> Vec<u8> {
        buf = set_buf_u64(buf, 4, self.meta.timestamp);
        buf = set_buf_u32(buf, 12, self.meta.key_size);
        buf = set_buf_u32(buf, 16, self.meta.value_size);
        buf = set_buf_u16(buf, 20, self.meta.flag);
        buf = set_buf_u32(buf, 22, self.meta.ttl);
        buf = set_buf_u32(buf, 26, self.meta.bucket_size);
        buf = set_buf_u16(buf, 30, self.meta.status);
        buf = set_buf_u16(buf, 32, self.meta.ds);
        buf = set_buf_u64(buf, 34, self.meta.tx_id);
        return buf;
    }

    pub fn is_zero(&self) -> bool {
        if self.meta.key_size == 0 && self.meta.value_size == 0 && self.meta.timestamp == 0 {
            return true;
        }
        return false;
    }
    
    pub fn get_crc(&self, buf: Vec<u8>) -> u32 {
        let mut digest = crc32fast::Hasher::new();
        digest.update(&buf[4..]);
        digest.update(&self.meta.bucket);
        digest.update(&self.key);
        digest.update(&self.value);
        return digest.finalize();
    }
}

/// Meta
#[derive(Debug, Clone)]
pub struct  Meta {
    pub key_size: u32,
    pub value_size: u32,
    pub timestamp: u64,
    pub ttl: u32,
    pub flag: u16,
    pub bucket: ::std::vec::Vec<u8>,
    pub bucket_size: u32,
    pub tx_id: u64,
    pub status: u16,
    pub ds: u16,
}

impl Meta {
    pub fn new() -> Meta {
        return Meta{
            key_size: 0,
            value_size: 0,
            timestamp: 0,
            ttl: 0,
            flag: 0,
            bucket: vec![],
            bucket_size: 0,
            tx_id: 0,
            status: 0,
            ds: 0,
        }
    }
}
