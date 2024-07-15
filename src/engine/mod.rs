use crate::errors::DBError;
/// Trait for a key value storage engine.
pub trait KvsEngine {
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&mut self, key: String, value: String) -> Result<Option<String>, DBError> ;

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&mut self, key: String) -> Result<Option<String>, DBError>;

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    fn remove(&mut self, key: String) -> Result<Option<String>, DBError> ;
}

pub mod kvs;
pub mod sled;

pub use self::kvs::KvStore;
pub use self::sled::SledKvsEngine;
