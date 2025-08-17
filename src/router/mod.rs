/// Used to handle routes in a Router
pub mod route;

#[allow(missing_docs)]
pub mod middleware;

use std::{
    collections::HashMap,
    fs::{self},
    sync::Arc,
    path::{Path, PathBuf},
};

pub use route::Route;

use crate::{
    request::Request, response::{Response, Status}, router::middleware::{run_middleware_and_route, Middleware}, Error
};

type RouteHandler = fn(Request, Response) -> Response;

/// Handles the routing of requests made by the client.
#[derive(Debug, Clone)]
pub struct Router {
    path: String,

    routes: HashMap<Route, RouteHandler>,
    routers: HashMap<String, Router>,

    middlewares: Vec<Arc<dyn Middleware>>,

    default_response: Option<Response>,

    not_found_handler: Option<RouteHandler>,

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
            middlewares: Vec::new(),
            default_response: None,
            not_found_handler: None,
            static_path: None,
        }
    }

    /// Handles a response for a given route
    pub fn handle_route(&mut self, route: Route, handler: RouteHandler) {
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

    /// Handles a not found route, this will be called when a route is not found
    pub fn handle_not_found(&mut self, handler: RouteHandler) {
        self.not_found_handler = Some(handler);
    }

    /// Adds a middleware to the router
    pub fn insert_middleware(&mut self, middleware: Arc<dyn Middleware>) {
        self.middlewares.push(middleware);
    }

    fn not_found_handler(&self, request: Request, response: Response) -> Result<Response, Error> {
        if let Some(handler_fn) = self.not_found_handler {
            let mut resp = handler_fn(request, response);
            resp.set_status(Status::NotFound);
            return Ok(resp);
        }

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
            .unwrap_or_else(|| Response::new(Status::OK));


        if let Some(handler) = self.routes.get(&request_route) {
            let middlewares_arc: Arc<[Arc<dyn Middleware>]> = self.middlewares.clone().into_boxed_slice().into();
            let handler_fn = *handler; 

            return Ok(
                run_middleware_and_route(middlewares_arc, request, response, 0, handler_fn)
            );
        }

        let route_segment = match path_str.split('/').nth(1) {
            Some(route) => route,
            None => {
                return self.not_found_handler(request, response);
            }
        };

        if let Some(subrouter) = self.routers.get(format!("/{route_segment}").as_str()) {
            return subrouter.handle_request(request);
        }

        macro_rules! check_unsafe_path {
            ($var:expr, $($unsafe_expr:expr),*) => {
                if $($var.contains($unsafe_expr) ||)* false {
                    return self.not_found_handler(request, response);
                }
            };
        }

        if let Some(static_path) = &self.static_path {
            check_unsafe_path!(path_str, "../", "..\\", ".\\", "~", "//", "\\", ":", "*");

            if let Ok(path) = fs::canonicalize(static_path) {
                let file_path = path.join(path_str.trim_start_matches('/'));

                if file_path.exists() {
                    let mut res = Response::new(Status::OK);
                    res.send_file(file_path)?;
                    return Ok(res);
                }
            }
        }

        self.not_found_handler(request, response)
    }
}

/// The middlewares of both routers cannot be compared
impl PartialEq for Router {
    fn eq(&self, other: &Self) -> bool {
        if self.routes != other.routes || self.path != other.path || self.routers != other.routers || self.default_response != other.default_response || self.static_path != other.static_path {
            return false;
        }
        return true;
    }
}

/// Errors that can occur when routing requests.
#[derive(Debug, thiserror::Error)]
pub enum RouterError {
    /// Route not found.
    #[error("Route not found: {0:?}")]
    RouteNotFound(Route),
}
