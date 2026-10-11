use crate::framework::http::request::request_line::method::Method;
use crate::framework::http::request::request_line::protocol::Protocol;
use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;

mod method;
mod protocol;

#[derive(Serialize)]
pub(crate) struct RequestLine<'a> {
    protocol: Protocol,
    method: Method,
    path: &'a str,
    path_params: Option<Vec<&'a str>>,
    query_params: Option<HashMap<&'a str, &'a str>>,
}

impl<'a> RequestLine<'a> {
    pub fn new(request_line_str: &'a str) -> Result<Self, &'a str> {
        let (method, uri, protocol) =
            Self::split_method_uri_and_protocol(request_line_str)?;
        let (path, path_params, query_params) =
            Self::split_path_and_params(uri);

        Ok(Self {
            protocol: Protocol::from_str(protocol)?,
            method: Method::from_str(method)?,
            path,
            path_params,
            query_params,
        })
    }

    fn split_method_uri_and_protocol(
        request_line_str: &str,
    ) -> Result<(&str, &str, &str), &str> {
        let parts: Vec<&str> = request_line_str.split_whitespace().collect();
        Ok((
            parts.get(0).ok_or("missing method.")?,
            parts.get(1).ok_or("missing url.")?,
            parts.get(2).ok_or("missing protocol.")?,
        ))
    }

    fn split_path_and_params(
        uri: &str,
    ) -> (&str, Option<Vec<&str>>, Option<HashMap<&str, &str>>) {
        if let Some(index) = uri.find('?') {
            let path = &uri[..index];
            let query = &uri[index + 1..];

            let query_parameter = Self::parse_query_params(query);

            (path, None, Some(query_parameter))
        } else {
            (uri, None, None)
        }
    }

    fn parse_query_params(query: &str) -> HashMap<&str, &str> {
        let mut params: HashMap<&str, &str> = HashMap::new();

        for pair in query.split('&') {
            let mut key_and_value = pair.splitn(2, '=');
            if let (Some(key), Some(value)) =
                (key_and_value.next(), key_and_value.next())
            {
                params.insert(key, value);
            }
        }

        params
    }
}
