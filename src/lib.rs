pub mod request;
pub mod response;

pub mod router;

use std::{io::{self, prelude::*, BufReader}, net::{self, TcpStream}};

use router::Router;

#[derive(Debug)]
pub struct HttpServer {
    listener: std::net::TcpListener,

    router: Option<router::Router>,
}

#[derive(Debug)]
pub struct HttpServerError {
    pub message: String,
}

impl ToString for HttpServerError {
    fn to_string(&self) -> String {
        format!("[HTTP Server Error]: {}", self.message)
    }
}

impl From<io::Error> for HttpServerError {
    fn from(error: io::Error) -> Self {
        HttpServerError {
            message: format!("IO Error: {}", error),
        }
    }
}

impl From<router::RouterError> for HttpServerError {
    fn from(error:router::RouterError) -> Self {
        HttpServerError {
            message: format!("Router Error on ({}): {}", error.route, error.error_message),
        }
    }
}

impl HttpServer {
    pub fn new(port: u16) -> Result<Self, HttpServerError> {
        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", port))?;

        let server = HttpServer { listener, router: None };

        Ok(server)
    }

    pub fn add_router(&mut self, router: router::Router) {
        self.router = Some(router);
    }

    pub fn listen(&self) -> Result<(), HttpServerError> {

        if self.router.is_none() {
            return Err(HttpServerError {
                message: String::from("Router not set"),
            });
        }

        for stream_result in self.listener.incoming() {
            let stream = stream_result?;

            let router = self.router.clone().unwrap();
            Self::handle_connection(stream, router)?;
        }

        Ok(())
    }

    fn handle_connection(mut stream: TcpStream, router: Router) -> Result<(), HttpServerError> {
        let mut buf_reader = BufReader::new(&mut stream);

        let mut request_string = String::new();
        let mut body_size = 0;

        loop {
        
            let mut line_str = String::new();
            let bytes_read = buf_reader.read_line(&mut line_str).unwrap();

            request_string.push_str(&line_str);

            if body_size == 0 && line_str.to_lowercase().contains("content-length") {
                let mut parts = line_str.split(':');
                let value = parts.nth(1).unwrap();

                body_size = value.trim().parse::<usize>().unwrap();
            }

            if line_str.trim().is_empty() || bytes_read == 0 {
                break;
            }
        }

        let mut body_bytes_buffer = vec![0; body_size];
        buf_reader.read_exact(&mut body_bytes_buffer).unwrap();

        request_string.push_str(&String::from_utf8_lossy(&body_bytes_buffer));

        let request = request::Request::try_from(request_string.as_str()).unwrap();

        let mut resp = router.handle_request(request)?;

        resp.pack();

        stream.write_all(resp.to_string().as_bytes()).unwrap();
        
        Ok(())
    }
}