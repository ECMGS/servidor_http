use std::fmt;
use std::sync::{Arc, mpsc, Mutex};

use crate::Error;
use crate::dispatcher::{Job, Dispatcher};

mod worker;

use worker::Worker;

/// Enum used to send messages to Workers by the ThreadPoolDispatcher
pub enum ThreadPoolMsg {
    /// Executes a job
    Run(Job),

    /// Shutdowns the thread
    Shutdown
}

/// This dispatcher uses a thread pool to handle connections. Each connection is handled in a single thread
pub struct ThreadPoolDispatcher {
    workers: Vec<Worker>,
    tx: mpsc::Sender<ThreadPoolMsg>,
    rx: Arc<Mutex<mpsc::Receiver<ThreadPoolMsg>>>
}

impl ThreadPoolDispatcher {
    
    /// Returns a new ThreadPoolDispatcher
    pub fn new(pool_size: usize) -> ThreadPoolDispatcher {
        let (tx, rx) = mpsc::channel::<ThreadPoolMsg>();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(pool_size);
        for id in 0..pool_size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        ThreadPoolDispatcher {tx, rx, workers}
    }

    /// Starts the threads of the thread pool
    pub fn run(&mut self) {
        for worker in &mut self.workers {
            worker.run();
        }
    }

}

impl Dispatcher for ThreadPoolDispatcher {
    fn dispatch (&self, job: Job) -> Result<(), Error> {
        self.tx.send(ThreadPoolMsg::Run(job)).unwrap();
        Ok(())
    }
}

impl fmt::Debug for ThreadPoolDispatcher {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThreadPoolDispatcher")
//            .field("workers", format!("{} workers", &self.workers)
            .finish()
   }
}
