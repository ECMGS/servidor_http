use std::collections::HashMap;

use crate::package;

pub use crate::package::Package;

/// Contains all the supported response status codes.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Status {
    // 2xx
    OK,
    Created,
    Accepted,

    // 3xx
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,

    // 4xx
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,

    // 5xx
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    HttpVersionNotSupported,

    // Other status codes
    Other(u16, String),
}

impl TryFrom<u16> for Status {
    type Error = &'static str;

    fn try_from(status_code: u16) -> Result<Self, Self::Error> {
        match status_code {
            // 2xx
            200 => Ok(Status::OK),
            201 => Ok(Status::Created),
            202 => Ok(Status::Accepted),

            // 3xx
            301 => Ok(Status::MovedPermanently),
            302 => Ok(Status::Found),
            303 => Ok(Status::SeeOther),
            304 => Ok(Status::NotModified),

            // 4xx
            400 => Ok(Status::BadRequest),
            401 => Ok(Status::Unauthorized),
            403 => Ok(Status::Forbidden),
            404 => Ok(Status::NotFound),
            405 => Ok(Status::MethodNotAllowed),

            // 5xx
            500 => Ok(Status::InternalServerError),
            501 => Ok(Status::NotImplemented),
            502 => Ok(Status::BadGateway),
            503 => Ok(Status::ServiceUnavailable),
            505 => Ok(Status::HttpVersionNotSupported),

            // Handle other status codes
            val => Ok(Status::Other(val, "Unknown".to_string())),
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            // 2xx
            Status::OK => "200 OK".to_string(),
            Status::Created => "201 Created".to_string(),
            Status::Accepted => "202 Accepted".to_string(),

            // 3xx
            Status::MovedPermanently => "301 Moved Permanently".to_string(),
            Status::Found => "302 Found".to_string(),
            Status::SeeOther => "303 See Other".to_string(),
            Status::NotModified => "304 Not Modified".to_string(),

            // 4xx
            Status::BadRequest => "400 Bad Request".to_string(),
            Status::Unauthorized => "401 Unauthorized".to_string(),
            Status::Forbidden => "403 Forbidden".to_string(),
            Status::NotFound => "404 Not Found".to_string(),
            Status::MethodNotAllowed => "405 Method Not Allowed".to_string(),

            // 5xx
            Status::InternalServerError => "500 Internal Server Error".to_string(),
            Status::NotImplemented => "501 Not Implemented".to_string(),
            Status::BadGateway => "502 Bad Gateway".to_string(),
            Status::ServiceUnavailable => "503 Service Unavailable".to_string(),
            Status::HttpVersionNotSupported => "505 HTTP Version Not Supported".to_string(),

            // Handle other status codes
            Status::Other(code, message) => format!("{} {}", code, message),
        }
    }
}

/// Struct responsible for handling the response of a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    /// Status of the response
    pub status: Status,

    headers: HashMap<String, String>,
    body: Option<String>,
}

package::generate_package_getters_setters!(Response[String]);

impl Response {
    /// Generates a new response with the given status.
    pub fn new(status: Status) -> Self {
        Response {
            status,
            headers: HashMap::new(),
            body: None,
        }
    }
}

impl Response {
    pub(crate) fn pack(&mut self) {
        let content_length = match self.body.as_ref() {
            Some(body) => body.len().to_string(),
            None => "0".to_string(),
        };

        self.add_header("Content-Length", &content_length);

        if !self.has_header("Content-Type") {
            self.add_header("Content-Type", "text/plain");
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut resp = format!("HTTP/1.1 {}\r\n", self.status.to_string());

        for (key, value) in &self.headers {
            resp.push_str(&format!("{}: {}\r\n", key, value));
        }

        resp.push_str("\r\n");

        if let Some(body) = &self.body {
            resp.push_str(body);
        }

        resp
    }
}
