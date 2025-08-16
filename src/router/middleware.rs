use std::{fmt::Debug, sync::Arc};

use crate::{request::Request, response::Response, router::{RouteHandler}};

pub type NextHandler = Box<dyn FnOnce(Request, Response) -> Response + Send + 'static>;

pub trait Middleware: Send + Sync + Debug + 'static {
    fn handle(&self, req: Request, res: Response, next: NextHandler) -> Response;
}

pub(crate) fn run_middleware_and_route(middlewares: Arc<[Arc<dyn Middleware>]>, req: Request, res: Response, index: usize, route_handler: RouteHandler) -> Response{
    if let Some(middleware) = middlewares.get(index) {
        let middlewares_clone = Arc::clone(&middlewares);
        let route_handler_copy = route_handler;

        let next: NextHandler = Box::new(move |req, res| {
            run_middleware_and_route(middlewares_clone, req, res, index + 1, route_handler_copy)
        });

        return middleware.handle(req, res, next);
    } 

    route_handler(req, res)
}
