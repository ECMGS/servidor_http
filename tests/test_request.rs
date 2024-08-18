use servidor_http::request;

#[test]
fn request_without_headers() {
    let req_str = "GET /index.html HTTP/1.1\n";
    let req = request::Request::try_from(req_str);

    assert_eq!(req, Ok(request::Request::new(request::RequestMethod::GET, "/index.html")));
}

#[test]
fn request_with_headers() {
    let req_str = "GET /index.html HTTP/1.1
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
Host: www.example.com
";
    let mut req = request::Request::new(request::RequestMethod::GET, "/index.html");
    req.add_header("User-Agent", "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)");
    req.add_header("Host", "www.example.com");

    assert_eq!(request::Request::try_from(req_str), Ok(req));
}

#[test]
fn request_with_body() {
    let req_str = "POST /index.html HTTP/1.1

This is the body of the request.";

    let mut req = request::Request::new(request::RequestMethod::POST, "/index.html");
    req.set_body("This is the body of the request.");

    assert_eq!(request::Request::try_from(req_str), Ok(req));
}