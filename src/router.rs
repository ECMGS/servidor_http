pub mod route;

use std::collections::HashMap;

pub use route::Route;

use crate::{request::Request, response::{ResponseStatus, Response}};

#[derive(Debug, Clone)]
pub struct Router {
    route: String,

    routes: HashMap<Route, fn(Request, Response) -> Response>,
    routers: HashMap<String, Router>,

    default_response: Option<Response>
}

#[derive(Debug)]
pub struct RouterError {
    pub route: String,
    pub error_message: String
}

impl Default for Router {
    fn default() -> Self {
        Router::new(String::from("/"))
    }
}

impl Router {
    pub fn new(route: String) -> Self {
        Router {
            route,
            routes: HashMap::new(),
            routers: HashMap::new(),
            default_response: None
        }
    }

    

    pub fn handle_route(&mut self, route: Route, handler: fn(Request, Response) -> Response) {
        self.routes.insert(route, handler);
    }

    pub fn handle_router(&mut self, router: Router) {
        self.routers.insert(router.route.clone(), router);
    }

    pub(crate) fn not_found_handler(request: Request) -> Result<Response, RouterError>{
        Err(RouterError {
            route: request.url,
            error_message: String::from("Route not found")
        })
    }

    pub(crate) fn handle_request(&self, request: Request) -> Result<Response, RouterError> {

        let mut route_str = request.url.trim_start_matches(self.route.as_str()).to_string();

        if !route_str.starts_with('/') {
            route_str.insert(0, '/');
        }

        let request_route = Route::new(request.method, &route_str);

        let response = match self.default_response.clone() {
            Some(res) => res,
            None => {
                Response::new(ResponseStatus::OK)
            }
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

        if let Some(subrouter) = self.routers.get(format!("/{}",subrouter_route).as_str()) {
            return subrouter.handle_request(request);
        }

        Self::not_found_handler(request) 
    }
}