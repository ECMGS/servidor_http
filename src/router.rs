#[allow(missing_docs)]
pub mod route;

use std::collections::HashMap;

pub use route::Route;

use crate::{
    request::Request,
    response::{Response, ResponseStatus},
    Error,
};

/// Handles the routing of requests made by the client.
#[derive(Debug, Clone)]
pub struct Router {
    route: String,

    routes: HashMap<Route, fn(Request, Response) -> Response>,
    routers: HashMap<String, Router>,

    default_response: Option<Response>,
}

impl Default for Router {
    fn default() -> Self {
        Router::new(String::from("/"))
    }
}

impl Router {
    /// Generates a new router with a root route.
    pub fn new(route: String) -> Self {
        Router {
            route,
            routes: HashMap::new(),
            routers: HashMap::new(),
            default_response: None,
        }
    }

    /// Handles a response for a given route
    pub fn handle_route(&mut self, route: Route, handler: fn(Request, Response) -> Response) {
        self.routes.insert(route, handler);
    }

    /// Routes the route to a subrouter
    pub fn handle_router(&mut self, router: Router) {
        self.routers.insert(router.route.clone(), router);
    }

    fn not_found_handler(request: Request) -> Result<Response, Error> {
        let route = Route::new(request.route.method, request.route.path.as_str());
        Err(Error::RouterError(RouterError::RouteNotFound(route)))
    }

    pub(crate) fn handle_request(&self, request: Request) -> Result<Response, Error> {
        let mut route_str = request
            .route
            .path
            .trim_start_matches(self.route.as_str())
            .to_string();

        if !route_str.starts_with('/') {
            route_str.insert(0, '/');
        }

        let request_route = Route::new(request.route.method, &route_str);

        let response = match self.default_response.clone() {
            Some(res) => res,
            None => Response::new(ResponseStatus::OK),
        };

        if let Some(handler) = self.routes.get(&request_route) {
            return Ok(handler(request, response));
        }

        let subrouter_route = match route_str.split('/').nth(1) {
            Some(route) => route,
            None => {
                return Self::not_found_handler(request);
            }
        };

        if let Some(subrouter) = self.routers.get(format!("/{}", subrouter_route).as_str()) {
            return subrouter.handle_request(request);
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
