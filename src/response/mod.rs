use std::{collections::HashMap, fmt::Display, path::Path};

use crate::package;

pub use crate::package::Package;

mod status;
pub(crate) mod file_mime;

pub use status::Status;
use crate::response::file_mime::*;

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

    /// Sets a new session cookie (with the HttpOnly flag).
    pub fn set_session_cookie(&mut self, name: &str, value: &str) {
        self.add_header("Set-Cookie", &format!("{}={}; HttpOnly", name, value));
    }

    /// Sets the body of the response to the contents of a file.
    pub fn send_file<P>(&mut self, path: P) -> Result<(), crate::Error>
    where
        P: AsRef<Path>,
    {
        let content = std::fs::read_to_string(&path)?;

        let file_extension = path
            .as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let content_type:&str=extension_to_mime(file_extension).unwrap();// Debugging
        let content_ext:&str=mime_to_extension("image/jpeg").unwrap();// Debugging
        println!("{}", content_ext);// Debugging

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

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut resp = format!("HTTP/1.1 {}\r\n", self.status);

        for (key, value) in &self.headers {
            resp.push_str(&format!("{}: {}\r\n", key, value));
        }

        resp.push_str("\r\n");

        if let Some(body) = &self.body {
            resp.push_str(body);
        }

        write!(f, "{}", resp)
    }
}
