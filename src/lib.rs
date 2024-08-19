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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    ServerError(#[from] ServerError),

    #[error(transparent)]
    RouterError(#[from] router::RouterError),

    #[error(transparent)]
    RequestError(#[from] request::RequestError),
}

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("HttpServer has no router attached")]
    NoRouterAttached,
}

impl HttpServer {
    pub fn new(port: u16) -> Result<Self, Error> {
        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", port))?;

        let server = HttpServer { listener, router: None };

        Ok(server)
    }

    pub fn attach_router(&mut self, router: router::Router) {
        self.router = Some(router);
    }

    pub fn listen(&self) -> Result<(), Error> {

        if self.router.is_none() {
            return Err(Error::ServerError(ServerError::NoRouterAttached));
        }

        for stream_result in self.listener.incoming() {
            let stream = stream_result?;

            let router = self.router.clone().unwrap();
            Self::handle_connection(stream, router)?;
        }

        Ok(())
    }

    fn handle_connection(mut stream: TcpStream, router: Router) -> Result<(), Error> {
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