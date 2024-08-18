use servidor_http::{request, router::{self, Router}, HttpServer};

fn main() {
    let mut server = HttpServer::new(8080).unwrap();

    let mut router = Router::new(String::from("/"));

    router.handle_route(router::Route::new(request::RequestMethod::GET, "/"), |req, mut res| {
        res.set_body(&format!("<h1>{}</h1>", req.body.unwrap_or(String::from("No body Received"))));
        res.add_header("Content-Type", "text/html");
        res
    });

    let mut sub_router = Router::new(String::from("/sub"));

    sub_router.handle_route(router::Route::new(request::RequestMethod::GET, "/"), |_, mut res| {
        res.set_body(&format!("<h1>In Subrouter</h1>"));
        res.add_header("Content-Type", "text/html");
        res
    });

    router.handle_router(sub_router);

    server.add_router(router);

    loop {
        if let Err(e) = server.listen() {
            if e.message.contains("Route not found") {
                continue;
            };
            panic!("Error: {}", e.message);
        }
        
    }

}