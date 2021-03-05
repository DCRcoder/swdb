/// FileMode 文件读写方式
#[derive(Debug, Clone)]
pub enum FileMode {
    IO = 1,
    MMAP = 2,
}
use crate::error::SWDBError;

/// FileManager 文件管理 trait
trait FileManager {
    fn write_at(b: Vec<u8>, offset: i64) -> Result<i64, SWDBError>;
    fn read_at(b: Vec<u8>, offset: i64) -> Result<i64, SWDBError>;
    fn sync();
    fn close();
}
