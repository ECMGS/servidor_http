#[allow(missing_docs)]
pub mod route;

use std::{
    collections::HashMap,
    fs::{self},
    path::{Path, PathBuf},
};

pub use route::Route;

use crate::{
    request::Request,
    response::{Response, Status},
    Error,
};

/// Handles the routing of requests made by the client.
#[derive(Debug, Clone)]
pub struct Router {
    path: String,

    routes: HashMap<Route, fn(Request, Response) -> Response>,
    routers: HashMap<String, Router>,

    default_response: Option<Response>,

    static_path: Option<PathBuf>,
}

impl Default for Router {
    fn default() -> Self {
        Router::new(String::from("/"))
    }
}

impl Router {
    /// Generates a new router with a root route.
    pub fn new(path: String) -> Self {
        Router {
            path,
            routes: HashMap::new(),
            routers: HashMap::new(),
            default_response: None,
            static_path: None,
        }
    }

    /// Handles a response for a given route
    pub fn handle_route(&mut self, route: Route, handler: fn(Request, Response) -> Response) {
        self.routes.insert(route, handler);
    }

    /// Routes the route to a subrouter
    pub fn handle_router(&mut self, router: Router) {
        self.routers.insert(router.path.clone(), router);
    }

    /// Static path to serve files from
    pub fn handle_static<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        self.static_path = Some(PathBuf::from(path.as_ref()));
    }

    fn not_found_handler(request: Request) -> Result<Response, Error> {
        let route = Route::new(request.path.method, request.path.path.as_str());
        Err(Error::RouterError(RouterError::RouteNotFound(route)))
    }

    pub(crate) fn handle_request(&self, request: Request) -> Result<Response, Error> {
        let mut path_str = request
            .path
            .path
            .trim_start_matches(self.path.as_str())
            .to_string();

        if !path_str.starts_with('/') {
            path_str.insert(0, '/');
        }

        let request_route = Route::new(request.path.method.clone(), &path_str);

        let response = self
            .default_response
            .clone()
            .unwrap_or_else(|| Response::new(Status::OK)); // Limpieza de código

        if let Some(handler) = self.routes.get(&request_route) {
            return Ok(handler(request, response));
        }

        let route_segment = match path_str.split('/').nth(1) {
            Some(route) => route,
            None => {
                return Self::not_found_handler(request);
            }
        };

        if let Some(subrouter) = self.routers.get(format!("/{}", route_segment).as_str()) {
            return subrouter.handle_request(request);
        }

        macro_rules! check_unsafe_path {
            ($var:expr, $($unsafe_expr:expr),*) => {
                if $($var.contains($unsafe_expr) ||)* false {
                    return Self::not_found_handler(request);
                }
            };
        }

        if let Some(static_path) = &self.static_path {
            check_unsafe_path!(path_str, "../", "..\\", ".\\", "~", "//", "\\", ":", "*");

            if let Ok(path) = fs::canonicalize(static_path) {
                let file_path = path.join(path_str.trim_start_matches('/'));

                if file_path.exists() {
                    let mut res = Response::new(Status::OK);
                    res.send_file(file_path).unwrap();
                    return Ok(res);
                }
            }
        }

        Self::not_found_handler(request)
    }
}

/// Errors that can occur when routing requests.
#[derive(Debug, thiserror::Error)]
pub enum RouterError {
    /// Route not found.
    #[error("Route not found: {0:?}")]
    RouteNotFound(Route),
}
