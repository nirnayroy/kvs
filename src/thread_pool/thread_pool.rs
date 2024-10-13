use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::convert::TryFrom;
use super::ThreadPool;
use crate::errors::DBError;

pub struct NaiveThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>, // Wrap sender in Option for graceful shutdown
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool for NaiveThreadPool{
    /// Creates a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    fn new(size: u32) -> Result<NaiveThreadPool, DBError>{
        if size == 0 {
            return Err(DBError::Server);
        }

        let (sender, receiver) = mpsc::channel();
        // We need Arc and Mutex to share ownership and allow safe mutable access.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(usize::try_from(size).unwrap());
        for id in 0..size {
            // Create threads and store them in the workers vector
            workers.push(Worker::new(id, Arc::clone(&receiver))?);
        }

        Ok(NaiveThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    /// Spawns a function (job) into the thread pool.
    ///
    /// The job will be queued and executed by one of the available threads.
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);

        // Send the job to the thread pool's workers
        if let Some(ref sender) = self.sender {
            sender.send(job).unwrap();
        }
    }
}

// Worker struct manages the individual threads in the pool
struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, DBError> {
        // Spawn a new thread
        let thread = thread::Builder::new()
            .spawn(move || {
                loop {
                    let job = receiver.lock().unwrap().recv();

                    match job {
                        Ok(job) => {
                            println!("Worker {} got a job; executing.", id);
                            job();
                        }
                        Err(_) => {
                            println!("Worker {} disconnected; shutting down.", id);
                            break;
                        }
                    }
                }
            });

        match thread {
            Ok(handle) => Ok(Worker {
                id: id,
                thread: Some(handle),
            }),
            Err(e) => Err(DBError::Server),
        }
    }
}

// Implementing Drop for graceful shutdown of the thread pool
impl Drop for NaiveThreadPool {
    fn drop(&mut self) {
        // Close the sending side of the channel
        drop(self.sender.take());

        // Wait for all workers to finish
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
