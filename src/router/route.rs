use crate::{request::RequestMethod};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Route {
    pub method: RequestMethod,
    pub path: String,
}

impl Route {
    pub fn new(method: RequestMethod, path: &str) -> Self {
        Route {
            method,
            path: String::from(path),
        }
    }
}