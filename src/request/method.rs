/// Contains all the supported request methods.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    Other(String),
}

macro_rules! gen_try_from_and_from {
    ($($method:expr => $request_type:expr),*) => {
        impl TryFrom<&str> for Method {
            type Error = &'static str;

            fn try_from(method_str: &str) -> Result<Self, Self::Error> {
                match method_str {
                    $($method => Ok($request_type),)*
                    _ => Err("Request met"),
                }
            }
        }

        impl Method {
            /// Generates a request method from a string. If the method is not supported, it will return [RequestMethod::Other] with the method string inside.
            /// Use preferably [RequestMethod::try_from] instead.
            pub fn from(method_str: &str) -> Self {
                match method_str {
                    $($method => $request_type,)*
                    _ => Method::Other(String::from(method_str)),
                }
            }
        }

    };
}

gen_try_from_and_from!(
    "GET" => Method::GET,
    "POST" => Method::POST,
    "PUT" => Method::PUT,
    "DELETE" => Method::DELETE,
    "HEAD" => Method::HEAD
);
