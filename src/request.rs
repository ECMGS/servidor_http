use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD
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
            _ => Err("Request met")
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: RequestMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

impl Request {
    pub fn new(request_type: RequestMethod, url: &str) -> Self {
        Request {
            method: request_type,
            url: url.to_string(),
            headers: HashMap::new(),
            body: None
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = Some(body.to_string());
    }
}

impl TryFrom<&str> for Request {
    type Error = &'static str;

    fn try_from(req: &str) -> Result<Self, Self::Error> {
        let mut lines = req.lines();

        let mut request = match lines.next() {
            Some(request_line) => {
                let mut request_line_parts = request_line.split_whitespace();

                let request_method_string = match request_line_parts.next() {
                    Some(method) => {
                        method
                    },
                    None => return Err("Invalid request")
                };

                let request_method = RequestMethod::try_from(request_method_string)?;

                let request_url = match request_line_parts.next() {
                    Some(url) => {
                        url
                    },
                    None => return Err("No url found")
                };

                let _http_version = match request_line_parts.next() {
                    Some(version) => {
                        version
                    },
                    None => return Err("No http version found")
                };

                if !_http_version.contains("HTTP/") {
                    return Err("HTTP version not supported");
                }

                Request::new(request_method, request_url)
            }
            None => {
                return Err("No request line found");
            }
        };

        while let Some(header) = lines.next() {
            if header.is_empty() {
                break;
            }

            let mut header_parts = header.splitn(2, ':');
            
            let header_key = match header_parts.next() {
                Some(key) => key,
                None => return Err("No header key found")
            };

            let header_value = match header_parts.next() {
                Some(value) => value.trim(),
                None => return Err("No header value found")
            };

            request.add_header(header_key, header_value);
        } 

        let body_collection = lines.collect::<Vec<&str>>();

        if !body_collection.is_empty() {
            request.set_body(&body_collection.join("\n"));
        }
        
        Ok(request)

    }
}