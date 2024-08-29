use std::collections::HashMap;

use super::RequestError;

/// Contains a list of cookies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CookieList {
    cookies: HashMap<String, String>,
}

impl TryFrom<&str> for CookieList {
    type Error = crate::Error;

    fn try_from(cookie_list_str: &str) -> Result<Self, Self::Error> {
        let mut cookie_list = CookieList::new();

        let trimmed_cookie_list_str = cookie_list_str.trim_start_matches("Cookie: ");
        let cookie_pairs = trimmed_cookie_list_str.split("; ");

        for cookie_pair in cookie_pairs {
            if cookie_pair.trim().is_empty() {
                break;
            }

            let mut splited_cookie_pair = cookie_pair.split('=');

            let cookie_key = match splited_cookie_pair.next() {
                Some(key) => key,
                None => {
                    return Err(crate::Error::RequestError(RequestError::CookieError(
                        String::from(cookie_list_str),
                    )))
                }
            };

            let cookie_value = match splited_cookie_pair.next() {
                Some(key) => key,
                None => {
                    return Err(crate::Error::RequestError(RequestError::CookieError(
                        String::from(cookie_list_str),
                    )))
                }
            }
            .trim_end_matches(';');

            cookie_list.set(cookie_key, cookie_value);
        }

        Ok(cookie_list)
    }
}

impl Default for CookieList {
    fn default() -> Self {
        Self::new()
    }
}

impl CookieList {
    /// Generates a new cookie list.
    pub fn new() -> Self {
        CookieList {
            cookies: HashMap::new(),
        }
    }

    /// Adds a new cookie to the list.
    pub fn set(&mut self, name: &str, value: &str) {
        self.cookies.insert(String::from(name), String::from(value));
    }

    /// Removes a cookie from the list.
    pub fn remove(&mut self, name: &str) {
        self.cookies.remove(name);
    }

    /// Returns the value of a cookie.
    pub fn get(&self, name: &str) -> Option<&String> {
        self.cookies.get(name)
    }

    /// Returns true if the cookie list contains the cookie.
    pub fn contains(&self, name: &str) -> bool {
        self.cookies.contains_key(name)
    }
}
