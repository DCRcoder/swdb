use std::io;
use failure::Fail;
#[derive(Fail, Debug)]
pub enum SWDBError {
    #[fail(display = "{}", _0)]
    IO(#[cause] io::Error),
}

impl From<io::Error> for SWDBError {
    fn from(err: io::Error) -> SWDBError {
        SWDBError::IO(err)
    }
}