use std::collections::HashMap;

use crate::package;
use crate::router::Route;

pub use crate::package::Package;

/// Contains all the supported request methods.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    Other(String),
}

macro_rules! gen_try_from_and_from {
    ($($method:expr => $request_type:expr),*) => {
        impl TryFrom<&str> for Method {
            type Error = &'static str;

            fn try_from(method_str: &str) -> Result<Self, Self::Error> {
                match method_str {
                    $($method => Ok($request_type),)*
                    _ => Err("Request met"),
                }
            }
        }

        impl Method {
            /// Generates a request method from a string. If the method is not supported, it will return [RequestMethod::Other] with the method string inside.
            /// Use preferablly [RequestMethod::try_from] instead.
            pub fn from(method_str: &str) -> Self {
                match method_str {
                    $($method => $request_type,)*
                    _ => Method::Other(String::from(method_str)),
                }
            }
        }

    };
}

gen_try_from_and_from!(
    "GET" => Method::GET,
    "POST" => Method::POST,
    "PUT" => Method::PUT,
    "DELETE" => Method::DELETE,
    "HEAD" => Method::HEAD
);

/// Represents a request made by a client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    /// The route of the request.
    pub path: Route,

    headers: HashMap<String, String>,
    body: Option<String>,
}

package::generate_package_getters_setters!(Request[String]);

impl Request {
    /// Generates a new request method, with the given method and path.
    pub fn new(method: Method, path: &str) -> Self {
        let route = Route::new(method, path);

        Request::from(route)
    }
}

impl From<Route> for Request {
    fn from(path: Route) -> Self {
        Request {
            path,
            headers: HashMap::new(),
            body: None,
        }
    }
}

impl TryFrom<&str> for Request {
    type Error = crate::Error;

    fn try_from(req: &str) -> Result<Self, Self::Error> {
        let mut lines = req.lines();

        let mut request = match lines.next() {
            Some(request_line) => {
                let mut request_line_parts = request_line.split_whitespace();

                let request_method_string = match request_line_parts.next() {
                    Some(method) => method,
                    None => {
                        return Err(crate::Error::RequestError(RequestError::InvalidRequest(
                            String::from(req),
                        )))
                    }
                };

                let request_method = match Method::try_from(request_method_string) {
                    Ok(method) => method,
                    Err(_) => {
                        return Err(crate::Error::RequestError(
                            RequestError::InvalidRequestMethod(String::from(request_method_string)),
                        ))
                    }
                };

                let request_path = match request_line_parts.next() {
                    Some(url) => url,
                    None => return Err(crate::Error::RequestError(RequestError::NoUrlFound)),
                };

                let http_version = match request_line_parts.next() {
                    Some(version) => version,
                    None => {
                        return Err(crate::Error::RequestError(RequestError::InvalidRequest(
                            String::from(req),
                        )))
                    }
                };

                if !http_version.contains("HTTP/") {
                    return Err(crate::Error::RequestError(
                        RequestError::HttpVersionNotSupported(String::from(http_version)),
                    ));
                }

                Request::new(request_method, request_path)
            }
            None => {
                return Err(crate::Error::RequestError(RequestError::InvalidRequest(
                    String::from(req),
                )));
            }
        };

        for header in lines.by_ref() {
            if header.is_empty() {
                break;
            }

            let mut header_parts = header.splitn(2, ':');

            let header_key = match header_parts.next() {
                Some(key) => key,
                None => {
                    return Err(crate::Error::RequestError(RequestError::InvalidHeader(
                        String::from(header),
                    )))
                }
            };

            let header_value = match header_parts.next() {
                Some(value) => value.trim(),
                None => {
                    return Err(crate::Error::RequestError(RequestError::InvalidHeader(
                        String::from(header),
                    )))
                }
            };

            request.add_header(header_key, header_value);
        }

        let body_collection = lines.collect::<Vec<&str>>();

        if !body_collection.is_empty() {
            request.set_body(body_collection.join("\n"));
        }

        Ok(request)
    }
}

/// Contains all the possible errors that can occur when handling a request.
#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    /// The request is invalid (Either is empty or lacks an http version).
    /// Caused by a user client
    #[error("Invalid request\nRaw data:\n{0}")]
    InvalidRequest(String),

    /// The reques method is invalid. Check [crate::request::RequestMethod] for valid methods.
    #[error("Invalid request method: {0}")]
    InvalidRequestMethod(String),

    /// No URL was found in the request.
    #[error("No URL found in request")]
    NoUrlFound,

    /// The HTTP version is not supported.
    #[error("HTTP version not supported: {0}")]
    HttpVersionNotSupported(String),

    /// The header is invalid (Doesn't follow the `"key":"value"` squeme).
    #[error("Invalid header")]
    InvalidHeader(String),
}
