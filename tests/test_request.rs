use servidor_http::package::Package;
use servidor_http::request;

#[test]
fn request_without_headers() {
    let req_str = "GET /index.html HTTP/1.1\n";
    let req = request::Request::try_from(req_str).unwrap();

    assert_eq!(
        req,
        request::Request::new(request::RequestMethod::GET, "/index.html")
    );
}

#[test]
fn request_with_headers() {
    let req_str = "GET /index.html HTTP/1.1
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
Host: www.example.com
";
    let mut req = request::Request::new(request::RequestMethod::GET, "/index.html");
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

    let mut req = request::Request::new(request::RequestMethod::POST, "/index.html");
    req.set_body(String::from("This is the body of the request."));

    assert_eq!(request::Request::try_from(req_str).unwrap(), req);
}

#[test]
fn empty_request() {
    let req_str = "";
    let req = request::Request::try_from(req_str);

    match req.unwrap_err() {
        servidor_http::Error::RequestError(
            servidor_http::request::RequestError::InvalidRequest(_),
        ) => assert!(true),
        _ => assert!(false),
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

                assert_eq!(req, request::Request::new(request::RequestMethod::$method, "/index.html"));
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
            servidor_http::request::RequestError::InvalidRequestMethod(mtd),
        ) => assert_eq!(mtd, method),
        _ => assert!(false),
    }
}
