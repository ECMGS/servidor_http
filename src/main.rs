use servidor_http::{
    package::Package,
    request::{self},
    router::{self, Router},
    Error, HttpServer,
};

fn main() {
    let mut server = HttpServer::new(8080).unwrap();

    let mut router = Router::new(String::from("/"));

    router.handle_route(
        router::Route::new(request::Method::GET, "/"),
        |req, mut res| {
            res.set_body(format!("<h1>{}</h1>", req.get_body_string()).into_bytes());
            res.add_header("Content-Type", "text/html");
            res
        },
    );

    router.handle_route(
        router::Route::new(request::Method::GET, "/test"),
        |_, mut res| {
            let path = "tests/res/test.html";
            res.send_file(path).unwrap();
            res
        },
    );

    router.handle_route(
        router::Route::new(request::Method::GET, "/redirect"),
        |_, mut res| {
            res.redirect("/test");
            res
        },
    );

    router.handle_route(
        router::Route::new(request::Method::GET, "/cookie"),
        |req, mut res| {
            let mut cookie = String::from("No cookie");

            let cookies = req.cookies;

            if let Some(cookie_value) = cookies.get("cookie") {
                cookie = cookie_value.clone();
            }

            res.set_body(format!("<h1>Cookie: {}</h1>", cookie).into_bytes());
            res.add_header("Content-Type", "text/html");
            res.set_session_cookie("cookie", "got cookie");
            res
        },
    );

    router.handle_static("./tests/res/static");

    let mut sub_router = Router::new(String::from("/query"));

    sub_router.handle_route(
        router::Route::new(request::Method::GET, "/test"),
        |req, mut res| {
            let query = req.query.unwrap();

            let say = query.get("say").unwrap();
            res.set_body(format!("<h1>In Subrouter</h1><p>got: {}</p>", say).into_bytes());
            res.add_header("Content-Type", "text/html");
            res
        },
    );

    router.handle_router(sub_router);

    server.attach_router(router);
    let mut error_message_reg: Option<router::Route> = None;
    loop {
        if let Err(Error::RouterError(router::RouterError::RouteNotFound(msg))) = server.listen() {
            if Some(&msg) != error_message_reg.as_ref() {
                println!("Route not found: {:?}", msg);
                error_message_reg = Some(msg);
            }
        } else {
            error_message_reg = None;
        }
    }
}
