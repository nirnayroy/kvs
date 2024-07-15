use std::io;
use failure::Fail;

#[derive(Fail,Debug)]
pub enum DBError {
    #[fail(display = "Serialization Error")]
    Serialize(#[cause] serde_json::Error),

    #[fail(display = "IO error")]
    Io(io::Error),

    #[fail(display = "Logging errro")]
    Log,

    #[fail(display = "Key not found")]
    NoKey,

    #[fail(display = "Server Error")]
    Server,
}

impl DBError {
    pub fn from_log_read() -> DBError {
        DBError::Log
    }

    pub fn no_key() -> DBError {
        DBError::NoKey
    }

    pub fn server() -> DBError {
        DBError::Server
    }
}
