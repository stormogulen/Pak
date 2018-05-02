use rusqlite;



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    CompressionError,

    DecompressionError,

    ResourceNotFound,

    InternalError,
}


impl From<rusqlite::Error> for Error {
    fn from(_: rusqlite::Error) -> Error {
        Error::InternalError
    }
}
