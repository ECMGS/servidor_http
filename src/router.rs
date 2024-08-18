pub mod route;

use std::collections::HashMap;

pub use route::Route;

use crate::{request::Request, response::{ResponseStatus, Response}};

#[derive(Debug, Clone)]
pub struct Router {
    route: String,

    routes: HashMap<Route, fn(Request, Response) -> Response>,

    default_response: Option<Response>
}

#[derive(Debug)]
pub struct RouterError {
    pub route: String,
    pub error_message: String
}

impl Router {
    pub fn new(route: String) -> Self {
        Router {
            route,
            routes: HashMap::new(),
            default_response: None
        }
    }

    pub fn default() -> Self {
        Router::new(String::from("/"))
    }

    pub fn handle_route(&mut self, route: Route, handler: fn(Request, Response) -> Response) {
        self.routes.insert(route, handler);
    }

    pub(crate) fn handle_request(&self, request: Request) -> Result<Response, RouterError> {

        let route_str = request.url.replace(&self.route, "/");

        let request_route = Route::new(request.method, &route_str);

        let response = match self.default_response.clone() {
            Some(res) => res,
            None => {
                let res = Response::new(ResponseStatus::OK);
                res
            }
        };

        match self.routes.get(&request_route) {
            Some(handler) => Ok(handler(request, response)),
            None => {Err(RouterError {
                route: route_str,
                error_message: String::from("Route not found")
            })}
        }
    }
}