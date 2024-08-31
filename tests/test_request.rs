use servidor_http::package::Package;
use servidor_http::request::{self, Method};

#[test]
fn request_without_headers() {
    let req_str = "GET /index.html HTTP/1.1\n";
    let req = request::Request::try_from(req_str).unwrap();

    assert_eq!(
        req,
        request::Request::new(Method::GET, "/index.html", None)
    );
}

#[test]
fn request_with_headers() {
    let req_str = "GET /index.html HTTP/1.1
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
Host: www.example.com
";
    let mut req = request::Request::new(Method::GET, "/index.html", None);
    req.add_header(
        "User-Agent",
        "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)",
    );
    req.add_header("Host", "www.example.com");

    assert_eq!(request::Request::try_from(req_str).unwrap(), req);
}

#[test]
fn request_with_body() {
    let req_str = "POST /index.html HTTP/1.1

This is the body of the request.";

    let mut req = request::Request::new(Method::POST, "/index.html", None);
    req.set_body(String::from("This is the body of the request."));

    assert_eq!(request::Request::try_from(req_str).unwrap(), req);
}

#[test]
fn empty_request() {
    let req_str = "";
    let req = request::Request::try_from(req_str);

    match req.unwrap_err() {
        servidor_http::Error::RequestError(
            request::RequestError::InvalidRequest(_),
        ) => (),
        _ => unreachable!(),
    }
}

macro_rules! generate_request_method_type_tests {
    (
        $($test_name:ident, $method:ident);*
    ) => {
       $(
            #[test]
            fn $test_name () {
                let req_str = format!("{} /index.html HTTP/1.1\n", stringify!($method));
                let req = request::Request::try_from(req_str.as_str()).unwrap();

                assert_eq!(req, request::Request::new(request::Method::$method, "/index.html", None));
            }
       )*
    };
}

generate_request_method_type_tests!(get_request, GET; post_request, POST; put_request, PUT; delete_request, DELETE; head_request, HEAD);

#[test]
fn request_with_invalid_method() {
    let method = "Invalid,";
    let req_str = format!("{} /index.html HTTP/1.1\n", method);
    let req = request::Request::try_from(req_str.as_str());

    match req.unwrap_err() {
        servidor_http::Error::RequestError(
            request::RequestError::InvalidRequestMethod(mtd),
        ) => assert_eq!(mtd, method),
        _ => unreachable!(),
    }
}

#[test]
fn weird_request_method() {
    let method_str = "ECHO";
    let method = Method::from(method_str);
    assert_eq!(method, Method::Other(method_str.to_string()));
}

#[test]
fn request_with_query() {
    let req_str = "GET /index.html?query=1&query2=2 HTTP/1.1\n";
    let req = request::Request::try_from(req_str).unwrap();

    assert_eq!(
        req.query
            .as_ref()
            .expect("No query found")
            .get("query")
            .expect("no query key found"),
        "1"
    );
    assert_eq!(
        req.query
            .as_ref()
            .expect("No query found")
            .get("query")
            .expect("no query key found"),
        "1"
    );
}

#[test]
fn request_with_cookies() {
    let req_str = "GET /index.html HTTP/1.1\r\nCookie: cookie1=value1; cookie2=value2;\r\n";

    let req = request::Request::try_from(req_str).unwrap();

    assert!(req.cookies.contains("cookie1") && req.cookies.contains("cookie2"));

    assert_eq!(
        req.cookies.get("cookie1").unwrap(),
        "value1"
    );
    assert_eq!(
        req.cookies.get("cookie2").unwrap(),
        "value2"
    );
}
