use servidor_http::{
    package::Package,
    request::{self},
    router::{self, Router},
    HttpServer,
};

fn main() {
    let mut server = HttpServer::new(8080).unwrap();

    let mut router = Router::new(String::from("/"));

    router.handle_route(
        router::Route::new(request::Method::GET, "/"),
        |req, mut res| {
            res.set_body(format!(
                "<h1>{}</h1>",
                req.get_body().unwrap_or(String::from("No body Received"))
            ));
            res.add_header("Content-Type", "text/html");
            res
        },
    );

    let mut sub_router = Router::new(String::from("/query"));

    sub_router.handle_route(
        router::Route::new(request::Method::GET, "/test"),
        |req, mut res| {
            let query = req.query.unwrap();

            let say = query.get("say").unwrap();
            res.set_body(format!("<h1>In Subrouter</h1><p>got: {}</p>", say));
            res.add_header("Content-Type", "text/html");
            res
        },
    );

    router.handle_router(sub_router);

    server.attach_router(router);

    loop {
        if let Err(servidor_http::Error::RouterError(
            servidor_http::router::RouterError::RouteNotFound(msg),
        )) = server.listen()
        {
            println!("Route not found: {:?}", msg);
        }
    }
}
