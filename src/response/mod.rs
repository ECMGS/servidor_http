use std::{collections::HashMap, fmt::Display, path::Path};

use crate::{package, BinaryRepresentation};

pub use crate::package::Package;

pub(crate) mod file_mime;
mod status;

use crate::response::file_mime::*;
pub use status::Status;

/// Struct responsible for handling the response of a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    /// Status of the response
    pub status: Status,

    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

package::generate_package_getters_setters!(Response[Vec<u8>]);

impl Response {
    /// Generates a new response with the given status.
    pub fn new(status: Status) -> Self {
        Response {
            status,
            headers: HashMap::new(),
            body: None,
        }
    }

    /// Redirects the user to the specified path.
    pub fn redirect(&mut self, location: &str) {
        self.add_header("Location", location);
        self.status = Status::MovedPermanently;
    }

    // Should be moved to the package trait
    /// Sets the body of the response to a string.
    pub fn set_body_string(&mut self, body: String) {
        self.set_body(body.into_bytes());
    }

    /// Sets a new session cookie (with the HttpOnly flag).
    pub fn set_session_cookie(&mut self, name: &str, value: &str) {
        self.add_header("Set-Cookie", &format!("{}={}; HttpOnly", name, value));
    }

    /// Sets the body of the response to the contents of a file.
    pub fn send_file<P>(&mut self, path: P) -> Result<(), crate::Error>
    where
        P: AsRef<Path>,
    {
        let content = std::fs::read(&path)?;

        let file_extension = path
            .as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let content_type: &str = extension_to_mime(file_extension).unwrap(); // Debugging

        self.add_header("Content-Type", content_type);

        self.set_body(content);

        Ok(())
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

/// Implementation of the Display trait for the Response struct. WILL REPLACE NON VALID ASCII CHARS WITH "�".
impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut resp = format!("HTTP/1.1 {}\r\n", self.status);

        for (key, value) in &self.headers {
            resp.push_str(&format!("{}: {}\r\n", key, value));
        }

        resp.push_str("\r\n");

        if let Some(body) = &self.body {
            resp.push_str(String::from_utf8_lossy(body).as_ref());
        }

        write!(f, "{}", resp)
    }
}

impl BinaryRepresentation for Response {
    fn to_binary(&self) -> Vec<u8> {
        let mut resp = format!("HTTP/1.1 {}\r\n", self.status).into_bytes();

        for (key, value) in &self.headers {
            resp.extend_from_slice(format!("{}: {}\r\n", key, value).as_bytes());
        }

        resp.extend_from_slice("\r\n".as_bytes());

        if let Some(body) = &self.body {
            resp.extend_from_slice(body);
        }

        resp
    }
}
