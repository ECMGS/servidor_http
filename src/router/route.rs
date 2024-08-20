use crate::request::RequestMethod;

/// Represents a route of a request made by a client.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Route {
    /// The method of the request, check [crate::request::RequestMethod] for supported methods.
    pub method: RequestMethod,

    /// The path of the request.
    pub path: String,
}

impl Route {
    /// Generates a new request from the given method and path.
    pub fn new(method: RequestMethod, path: &str) -> Self {
        Route {
            method,
            path: String::from(path),
        }
    }
}
