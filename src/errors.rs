use std::io;

#[derive(Debug)]
pub enum DBError {
    Serialize(serde_json::Error),
    Io(io::Error),
    Log,
    NoKey,
}

impl DBError {
    pub fn from_log_read() -> DBError {
        DBError::Log
    }

    pub fn no_key() -> DBError {
        DBError::NoKey
    }
}
