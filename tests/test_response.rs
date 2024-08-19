use servidor_http::response;

#[test]
fn basic_response_to_string() {
    let response = response::Response::new(response::ResponseStatus::OK);

    let response_str = "HTTP/1.1 200 OK\r\n\r\n";
    assert_eq!(response.to_string(), response_str);
}

#[test]
fn response_with_headers_to_string() {
    let mut response = response::Response::new(response::ResponseStatus::OK);
    response.add_header("Content-Type", "text/html");
    response.add_header("Server", "Servidor HTTP");

    let response_str = response.to_string();
    let mut lines = response_str.lines();

    assert_eq!(lines.next().unwrap(), "HTTP/1.1 200 OK");

    let headers = vec!["Content-Type: text/html", "Server: Servidor HTTP"];
    let mut missing_meaders = false;

    for header in headers {
        if !response_str.contains(header) {
            missing_meaders = true;
            break;
        }
    }

    assert!(!missing_meaders);
}

#[test]
fn response_with_body_to_string() {
    let mut response = response::Response::new(response::ResponseStatus::OK);
    response.set_body("Hello, world!");

    let response_str = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
    assert_eq!(response.to_string(), response_str);
}

#[test]
fn response_with_headers_and_body_to_string() {
    let mut response = response::Response::new(response::ResponseStatus::OK);
    response.add_header("Content-Type", "text/html");
    response.add_header("Server", "Servidor HTTP");

    let body = "Test body";

    response.set_body(body);

    let response_str = response.to_string();
    let mut lines = response_str.lines();

    assert_eq!(lines.next().unwrap(), "HTTP/1.1 200 OK");

    let headers = vec!["Content-Type: text/html", "Server: Servidor HTTP"];
    let mut missing_meaders = false;

    for header in headers {
        if !response_str.contains(header) {
            missing_meaders = true;
            break;
        }
    }

    assert!(!missing_meaders);

    assert_eq!(response_str.split("\r\n\r\n").last().unwrap(), body);
}
