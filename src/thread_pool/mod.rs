use std::u32;

use crate::errors::DBError;


/// Trait for a key value storage engine.
pub trait ThreadPool {
    fn new(n_threads: u32)  ->  Result<Self, DBError> where Self: Sized ;

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static ;
}

pub mod thread_pool;

// pub use self::ThreadPool;