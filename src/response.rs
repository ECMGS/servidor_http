use std::{collections::HashMap, fmt::Display, path::Path};

use crate::package;

use crate::file_mime::{extension_to_mime, mime_to_extension};
pub use package::Package;

/// Contains all the supported response status codes.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Status {
    // 1xx
    Continue,
    SwitchingProtocol,
    Processing,
    EarlyHints,
    Checkpoint,

    // 2xx
    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,

    // 3xx
    MultipleChoice,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    Unused,
    TemporaryRedirect,
    PermanentRedirect,

    // 4xx
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RequestedRangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // 5xx
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    BandwidthLimitExceeded,
    NotExtended,
    NetworkAuthenticationRequired,
    NotUpdated,
    VersionMismatch,

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

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            // 1xx
            Status::Continue => "100 Continue".to_string(),
            Status::SwitchingProtocol => "101 Switching Protocols".to_string(),
            Status::Processing => "102 Processing".to_string(),
            Status::EarlyHints => "103 EarlyHints".to_string(),
            Status::Checkpoint => "103 Checkpoint".to_string(),

            // 2xx
            Status::OK => "200 OK".to_string(),
            Status::Created => "201 Created".to_string(),
            Status::Accepted => "202 Accepted".to_string(),
            Status::NonAuthoritativeInformation => "203 Non-Authoritative Information".to_string(),
            Status::NoContent => "204 No Content".to_string(),
            Status::ResetContent => "205 Reset Content".to_string(),
            Status::PartialContent => "206 Partial Content".to_string(),
            Status::MultiStatus => "207 Multi-Status".to_string(),
            Status::AlreadyReported => "208 Already Reported".to_string(),
            Status::IMUsed => "226 IM Used".to_string(),

            // 3xx
            Status::MultipleChoice => "300 Multiple Choices".to_string(),
            Status::MovedPermanently => "301 Moved Permanently".to_string(),
            Status::Found => "302 Found".to_string(),
            Status::SeeOther => "303 See Other".to_string(),
            Status::NotModified => "304 Not Modified".to_string(),
            Status::UseProxy => "305 Use Proxy".to_string(),
            Status::Unused => "306 Unused".to_string(),
            Status::TemporaryRedirect => "307 Temporary Redirect".to_string(),
            Status::PermanentRedirect => "308 Permanent Redirect".to_string(),

            // 4xx
            Status::BadRequest => "400 Bad Request".to_string(),
            Status::Unauthorized => "401 Unauthorized".to_string(),
            Status::PaymentRequired => "402 Payment Required".to_string(),
            Status::Forbidden => "403 Forbidden".to_string(),
            Status::NotFound => "404 Not Found".to_string(),
            Status::MethodNotAllowed => "405 Method Not Allowed".to_string(),
            Status::NotAcceptable => "406 Not Acceptable".to_string(),
            Status::ProxyAuthenticationRequired => "407 Proxy Authentication Required".to_string(),
            Status::RequestTimeout => "408 Request Timeout".to_string(),
            Status::Conflict => "409 Conflict".to_string(),
            Status::Gone => "410 Gone".to_string(),
            Status::LengthRequired => "411 Length Required".to_string(),
            Status::PreconditionFailed => "412 Precondition Failed".to_string(),
            Status::PayloadTooLarge => "413 Payload/Content Too Large".to_string(),
            Status::URITooLong => "414 URI Too Long".to_string(),
            Status::UnsupportedMediaType => "415 Unsupported Media Type".to_string(),
            Status::RequestedRangeNotSatisfiable => "416 Requested Range Not Satisfiable".to_string(),
            Status::ExpectationFailed => "417 Expectation Failed".to_string(),
            Status::ImATeapot => "418 I'm A Teapot".to_string(),
            Status::MisdirectedRequest => "421 Misdirected Request".to_string(),
            Status::UnprocessableEntity => "422 Unprocessable Entity".to_string(),
            Status::Locked => "423 Locked".to_string(),
            Status::FailedDependency => "424 Failed Dependency".to_string(),
            Status::TooEarly => "425 Too Early".to_string(),
            Status::UpgradeRequired => "426 Upgrade Required".to_string(),
            Status::PreconditionRequired => "428 Precondition Required".to_string(),
            Status::TooManyRequests => "429 Too Many Requests".to_string(),
            Status::RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large".to_string(),
            Status::UnavailableForLegalReasons => "451 Unavailable For Legal Reasons".to_string(),

            // 5xx
            Status::InternalServerError => "500 Internal Server Error".to_string(),
            Status::NotImplemented => "501 Not Implemented".to_string(),
            Status::BadGateway => "502 Bad Gateway".to_string(),
            Status::ServiceUnavailable => "503 Service Unavailable".to_string(),
            Status::GatewayTimeout => "504 Gateway Timeout".to_string(),
            Status::HttpVersionNotSupported => "505 HTTP Version Not Supported".to_string(),
            Status::VariantAlsoNegotiates => "506 Variant Also Negotiates".to_string(),
            Status::InsufficientStorage => "507 Insufficient Storage".to_string(),
            Status::LoopDetected => "508 Loop Detected".to_string(),
            Status::BandwidthLimitExceeded => "509 Bandwidth Limit Exceeded".to_string(),
            Status::NotExtended => "510 Not Extended".to_string(),
            Status::NetworkAuthenticationRequired => "511 Network Authentication Required".to_string(),
            Status::NotUpdated => "512 Not Updated".to_string(),
            Status::VersionMismatch => "513 Version Mismatch".to_string(),

            // Handle other status codes
            Status::Other(code, message) => format!("{} {}", code, message),
        };
        write!(f, "{}", str)
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
