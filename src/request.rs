use std::collections::HashMap;

use crate::package::{self, Package};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
}

impl TryFrom<&str> for RequestMethod {
    type Error = &'static str;

    fn try_from(req: &str) -> Result<Self, Self::Error> {
        match req {
            "GET" => Ok(RequestMethod::GET),
            "POST" => Ok(RequestMethod::POST),
            "PUT" => Ok(RequestMethod::PUT),
            "DELETE" => Ok(RequestMethod::DELETE),
            "HEAD" => Ok(RequestMethod::HEAD),
            _ => Err("Request met"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: RequestMethod,
    pub url: String,

    headers: HashMap<String, String>,
    body: Option<String>,
}

package::generate_package_getters_setters!(Request[String]);

impl Request {
    pub fn new(request_type: RequestMethod, url: &str) -> Self {
        Request {
            method: request_type,
            url: url.to_string(),
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

                let request_method = match RequestMethod::try_from(request_method_string) {
                    Ok(method) => method,
                    Err(_) => {
                        return Err(crate::Error::RequestError(
                            RequestError::InvalidRequestMethod(String::from(request_method_string)),
                        ))
                    }
                };

                let request_url = match request_line_parts.next() {
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

                Request::new(request_method, request_url)
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

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("Invalid request\nRaw data:\n{0}")]
    InvalidRequest(String),

    #[error("Invalid request method: {0}")]
    InvalidRequestMethod(String),

    #[error("No URL found in request")]
    NoUrlFound,

    #[error("HTTP version not supported: {0}")]
    HttpVersionNotSupported(String),

    #[error("Invalid header")]
    InvalidHeader(String),
}
