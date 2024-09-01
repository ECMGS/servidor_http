#![warn(missing_docs)]

//! Simple HTTP server crate that allows you to create a server and attach a router to it. The router will handle the requests and return the responses. The server listens on a given port and handles the requests using the attached router.

/// Contains the [package::Package] trait and its implementations for the [request::Request] and [response::Response] structs.
pub mod package;

/// Contains the [request::Request] struct, its implementations and [request::RequestError] error handling enum.
pub mod request;

/// Contains the [response::Response] struct, its implementations and [response::Response] error handling enum.
pub mod response;

/// Contains the [Router] struct, its implementations and [router::RouterError] error handling enum.
pub mod router;

use std::{
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use router::Router;

/// Struct that represents an HTTP server, it listens on a given port and handles requests from a given router. If no router is attached, it will return an error when calling the handle_connection() method.
#[derive(Debug)]
pub struct HttpServer {
    listener: TcpListener,
    router: Option<Router>,
}

/// Possible errors that can occur when using the crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Checkout [io::Error] for more details
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Checkout [ServerError] for more details
    #[error(transparent)]
    ServerError(#[from] ServerError),

    /// Checkout [router::RouterError] for more details
    #[error(transparent)]
    RouterError(#[from] router::RouterError),

    /// Checkout [request::RequestError] for more details
    #[error(transparent)]
    RequestError(#[from] request::RequestError),
}

/// Possible errors that can occur when using the [HttpServer] struct.
#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    /// Error that occurs when trying to handle a connection without a router attached. Consider calling the [HttpServer::attach_router] method before calling the [HttpServer::listen] method.
    ///
    /// # How this error can be produced
    ///
    /// ```rust
    /// use servidor_http::{HttpServer, Error, ServerError};
    ///
    /// let mut server = HttpServer::new(8080).unwrap();
    ///
    /// if let Err(Error::ServerError(ServerError::NoRouterAttached)) = server.listen() {
    ///     println!("Router need to be attached before listening for new connections");
    /// }
    ///
    /// ```
    #[error("HttpServer has no router attached")]
    NoRouterAttached,
}

/// # Example
///
/// ```no_run
/// use servidor_http::{HttpServer, router::{Route, Router}, request::{Request, Method, Package}, Error};
///
/// let mut server = HttpServer::new(8080).unwrap();
///
/// let mut router = Router::new(String::from("/"));
///
/// router.handle_route(
///     Route::new(Method::GET, "/"),
///     |req, mut res| {
///         res.set_body_string(String::from("Hello World"));
///         res
///     }
/// );
///
/// server.attach_router(router);
///
/// server.listen().unwrap();
///
/// ```
impl HttpServer {
    /// Creates a new instance of the [HttpServer] struct that listens on the given port. Can return an io error if the port is already in use
    pub fn new(port: u16) -> Result<Self, Error> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;

        let server = HttpServer {
            listener,
            router: None,
        };
        Ok(server)
    }

    /// Attaches a router to the server, the router will handle the requests and return the response to the client.
    pub fn attach_router(&mut self, router: Router) {
        self.router = Some(router);
    }

    /// Listens for incoming connections and handles them using the attached router. If no router is attached, it will return an error.
    ///
    /// **This method will enter a loop to check if any client has connected and will not return until an unhandled error appears**
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

        let mut request_bytes: Vec<u8> = Vec::new();
        let mut body_size = 0;

        loop {
            let mut line_str = String::new();
            let bytes_read = buf_reader.read_line(&mut line_str)?;

            let _ = request_bytes.write(line_str.as_bytes());

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
        buf_reader.read_exact(&mut body_bytes_buffer)?;

        let _ = request_bytes.write(&body_bytes_buffer);

        let request = request::Request::try_from(request_bytes)?;

        let mut resp = router.handle_request(request)?;

        resp.pack();

        stream.write_all(&resp.to_binary())?;

        Ok(())
    }
}

/// Trait that represents a binary representation of a struct. It should return a Vec<u8> with the binary representation of the struct. Used to send responses to the client.
pub trait BinaryRepresentation {
    /// Returns a Vec<u8> with the binary representation of the struct.
    fn to_binary(&self) -> Vec<u8>;
}
