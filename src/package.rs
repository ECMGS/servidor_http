use std::collections::HashMap;

pub trait Package<BodyType> {
    fn get_header_list(&self) -> HashMap<String, String>;
    fn set_header_list(&mut self, headers: HashMap<String, String>);

    fn set_body(&mut self, body: BodyType);
    fn get_body(&self) -> Option<BodyType>;

    fn add_header(&mut self, key: &str, value: &str) {
        let mut header_list = self.get_header_list();
        header_list.insert(key.to_string(), value.to_string());
        self.set_header_list(header_list);
    }

    fn has_header(&self, key: &str) -> bool {
        self.get_header_list().contains_key(key)
    }

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
