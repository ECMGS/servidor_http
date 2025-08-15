use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::dispatcher::thread_pool_dispatcher::ThreadPoolMsg;

/// Worker for the ThreadPoolDispatcherStruct
pub struct Worker {
    id: usize,
    rx: Arc<Mutex<mpsc::Receiver<ThreadPoolMsg>>>
}

impl Worker {

    /// Returns a new Worker
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<ThreadPoolMsg>>>) -> Worker {
        Worker {
            id,
            rx
        }
    }

    /// Generates a thread for the worker and starts executing jobs
    pub fn run(&mut self) {
        let mut thread_builder = thread::Builder::new();
        thread_builder = thread_builder.name(format!("thread-${}", self.id));
        let rx_clone = Arc::clone(&self.rx);
        thread_builder.spawn(move || {

            loop {
                let msg = {
                    let rx = rx_clone.lock().unwrap();
                    rx.recv().unwrap()
                };

                match msg {
                    ThreadPoolMsg::Run(job) => {
                        job().unwrap();
                    },
                    ThreadPoolMsg::Shutdown => {
                        panic!("Not implemented");
                    }
                }
            }
        }).unwrap(); 
    }
}
