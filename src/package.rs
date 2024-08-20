use std::collections::HashMap;

/// Trait used by the [crate::request::Request] and [crate::response::Response] structs in order to handle the headers and body.
pub trait Package<BodyType> {
    /// Returns a HashMap containing the headers of the package.
    fn get_header_list(&self) -> HashMap<String, String>;

    /// Sets all the headers of the package.
    fn set_header_list(&mut self, headers: HashMap<String, String>);

    /// Sets the body of the package.
    fn set_body(&mut self, body: BodyType);

    /// Returns the body of the package if it exists.
    fn get_body(&self) -> Option<BodyType>;

    /// Adds a header to the package.
    fn add_header(&mut self, key: &str, value: &str) {
        let mut header_list = self.get_header_list();
        header_list.insert(key.to_string(), value.to_string());
        self.set_header_list(header_list);
    }

    /// Checks if the package has a header with the given key.
    fn has_header(&self, key: &str) -> bool {
        self.get_header_list().contains_key(key)
    }

    /// Returns the value of the header with the given key.
    fn remove_header(&mut self, key: &str) {
        self.get_header_list().remove(key);
    }
}

macro_rules! generate_package_getters_setters {
    ($type:ty[$body_type:ty]) => {
        impl Package<$body_type> for $type {
            fn get_header_list(&self) -> HashMap<String, String> {
                self.headers.clone()
            }

            fn set_header_list(&mut self, headers: HashMap<String, String>) {
                self.headers = headers;
            }

            fn set_body(&mut self, body: $body_type) {
                self.body = Some(body);
            }

            fn get_body(&self) -> Option<$body_type> {
                self.body.clone()
            }
        }
    };
}

pub(crate) use generate_package_getters_setters;
