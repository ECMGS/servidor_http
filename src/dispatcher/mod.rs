use std::fmt::Debug;
use std::thread;

use crate::Error;

/// This type is used to handle connections, which for the dispatcher are jobs
pub type Job = Box<dyn FnOnce() -> Result<(), Error> + Send>;

#[doc(hidden)]
pub mod thread_pool_dispatcher;

/// Used to dispatch connections by the server
pub trait Dispatcher: Send + Sync + Debug +  'static {
    /// How the new connection is handled when accepted
    fn dispatch (&self, job: Job) -> Result<(), Error>;
}

/// Use to dispatch connections in a single thread. Not recommended
#[derive(Debug)]
pub struct SingleThreadDispatcher;

impl Dispatcher for SingleThreadDispatcher {
    fn dispatch (&self, job: Job) -> Result<(), Error> {
        job()
    }
}

/// use to dispatch connectins by forking the thread
#[derive(Debug)]
pub struct ForkDispatcher;

impl Dispatcher for ForkDispatcher{
    fn dispatch (&self, job: Job) -> Result<(), Error> {
        thread::spawn(move || {
            if let Err(e) = job() {
               eprintln!("[Error] - Error when executing job: {e}"); 
            }
        });

        Ok(())
    }
}
