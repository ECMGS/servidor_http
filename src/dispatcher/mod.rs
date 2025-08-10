use crate::Error;

use std::fmt::Debug;

pub trait Dispatcher: Send + Sync + Debug +  'static {
    fn dispatch (&self, job: Box<dyn FnOnce() -> Result<(), Error> + Send>) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct SingleThreadDispatcher;

impl Dispatcher for SingleThreadDispatcher {
    fn dispatch (&self, job: Box<dyn FnOnce() -> Result<(), Error> + Send>) -> Result<(), Error> {
        job()
    }
}
