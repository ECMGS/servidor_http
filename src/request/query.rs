use std::collections::HashMap;

use crate::request::RequestError;
use crate::Error;

/// Query parameters of a request, represented as a key-value pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    query: HashMap<String, String>,
}

impl Query {
    /// Returns an Ok(String) with the parameter if the value is found, else it returns None.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.query.get(key)
    }

    /// Sets a new key-value pair in the query. Returns the previous value if the key already exists.
    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.query.insert(String::from(key), String::from(value))
    }

    /// Removes a key-value pair from the query. Returns the value if the key exists.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.query.remove(key)
    }

    /// Returns true if the query contains the key.
    pub fn contains(&self, key: &str) -> bool {
        self.query.contains_key(key)
    }
}

macro_rules! parse_query_string {
    ($query_string:expr; $($replaced:expr => $replacement:expr),*) => {{
        let mut new_query_string = String::from($query_string);

        $(
            new_query_string = new_query_string.replace($replaced, $replacement);
        )*

        new_query_string
    }};
}

impl TryFrom<&str> for Query {
    type Error = crate::Error;

    fn try_from(query_string: &str) -> Result<Self, Self::Error> {
        let mut query_map = HashMap::new();

        let trimmed_query_string = query_string.trim_start_matches('?');

        let parsed_query_string = parse_query_string!(
            trimmed_query_string;
            "%20" => " ",
            "%21" => "!",
            "%23" => "#",
            "%24" => "$",
            "%26" => "&",
            "%27" => "'",
            "%28" => "(",
            "%29" => ")",
            "%2A" => "*",
            "%2B" => "+",
            "%2C" => ",",
            "%2F" => "/",
            "%3A" => ":",
            "%3B" => ";",
            "%3D" => "=",
            "%3F" => "?",
            "%40" => "@",
            "%5B" => "[",
            "%5D" => "]",
            "+" => " "
        );

        for query_pair in parsed_query_string.split('&') {
            let mut query_pair = query_pair.split('=');

            let key = query_pair
                .next()
                .ok_or(Error::RequestError(RequestError::QueryError(String::from(
                    query_string,
                ))))?;
            let value = query_pair
                .next()
                .ok_or(Error::RequestError(RequestError::QueryError(String::from(
                    query_string,
                ))))?;

            query_map.insert(String::from(key), String::from(value));
        }

        let query = Query { query: query_map };

        Ok(query)
    }
}
