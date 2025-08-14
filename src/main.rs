use servidor_http::{
    package::Package,  
    request::{self},
    router::{self, Router},
    dispatcher::thread_pool_dispatcher,
    HttpServer,
};

fn main() {
    let mut server = HttpServer::new(8080).unwrap();

    let mut tpd = thread_pool_dispatcher::ThreadPoolDispatcher::new(5);
    tpd.run();
    server.set_dispatcher(tpd);

    let mut router = Router::new(String::from("/"));

    router.handle_route(
        router::Route::new(request::Method::GET, "/"),
        |req, mut res| {
            let remote_ip = req.get_remote_ip().unwrap();
            res.set_body(format!("<h1>{}</h1><p>Remote ip: {remote_ip}</p>", req.get_body_string()).into_bytes());
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

            res.set_body(format!("<h1>Cookie: {cookie}</h1>").into_bytes());
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
            res.set_body(format!("<h1>In Subrouter</h1><p>got: {say}</p>").into_bytes());
            res.add_header("Content-Type", "text/html");
            res
        },
    );

    router.handle_router(sub_router);

    router.handle_not_found(|_, mut res| {
        res.set_body_string(String::from("404 Not Found"));
        res
    });

    server.attach_router(router);

    server.listen().unwrap();
}
