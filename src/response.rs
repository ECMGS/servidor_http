use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResponseStatus {
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

impl TryFrom<u16> for ResponseStatus {
    type Error = &'static str;

    fn try_from(status_code: u16) -> Result<Self, Self::Error> {
        match status_code {
            // 2xx
            200 => Ok(ResponseStatus::OK),
            201 => Ok(ResponseStatus::Created),
            202 => Ok(ResponseStatus::Accepted),

            // 3xx
            301 => Ok(ResponseStatus::MovedPermanently),
            302 => Ok(ResponseStatus::Found),
            303 => Ok(ResponseStatus::SeeOther),
            304 => Ok(ResponseStatus::NotModified),

            // 4xx
            400 => Ok(ResponseStatus::BadRequest),
            401 => Ok(ResponseStatus::Unauthorized),
            403 => Ok(ResponseStatus::Forbidden),
            404 => Ok(ResponseStatus::NotFound),
            405 => Ok(ResponseStatus::MethodNotAllowed),

            // 5xx
            500 => Ok(ResponseStatus::InternalServerError),
            501 => Ok(ResponseStatus::NotImplemented),
            502 => Ok(ResponseStatus::BadGateway),
            503 => Ok(ResponseStatus::ServiceUnavailable),
            505 => Ok(ResponseStatus::HttpVersionNotSupported),

            // Handle other status codes
            val => Ok(ResponseStatus::Other(val, "Unknown".to_string())),
        }
    }
}

impl ToString for ResponseStatus {
    fn to_string(&self) -> String {
        match self {
            // 2xx
            ResponseStatus::OK => "200 OK".to_string(),
            ResponseStatus::Created => "201 Created".to_string(),
            ResponseStatus::Accepted => "202 Accepted".to_string(),

            // 3xx
            ResponseStatus::MovedPermanently => "301 Moved Permanently".to_string(),
            ResponseStatus::Found => "302 Found".to_string(),
            ResponseStatus::SeeOther => "303 See Other".to_string(),
            ResponseStatus::NotModified => "304 Not Modified".to_string(),

            // 4xx
            ResponseStatus::BadRequest => "400 Bad Request".to_string(),
            ResponseStatus::Unauthorized => "401 Unauthorized".to_string(),
            ResponseStatus::Forbidden => "403 Forbidden".to_string(),
            ResponseStatus::NotFound => "404 Not Found".to_string(),
            ResponseStatus::MethodNotAllowed => "405 Method Not Allowed".to_string(),

            // 5xx
            ResponseStatus::InternalServerError => "500 Internal Server Error".to_string(),
            ResponseStatus::NotImplemented => "501 Not Implemented".to_string(),
            ResponseStatus::BadGateway => "502 Bad Gateway".to_string(),
            ResponseStatus::ServiceUnavailable => "503 Service Unavailable".to_string(),
            ResponseStatus::HttpVersionNotSupported => "505 HTTP Version Not Supported".to_string(),

            // Handle other status codes
            ResponseStatus::Other(code, message) => format!("{} {}", code, message),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub status: ResponseStatus,

    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Response {
    pub fn new(status: ResponseStatus) -> Self {
        Response {
            status,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn has_header(&self, key: &str) -> bool {
        self.headers.contains_key(key)
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = Some(body.to_string());
    }

    pub fn pack(&mut self) {
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
