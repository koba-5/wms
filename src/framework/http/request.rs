use crate::framework::http::request::request_header::RequestHeader;
use crate::framework::http::request::request_line::RequestLine;
use serde::Serialize;
use std::io::Lines;

mod request_header;
mod request_line;

#[derive(Serialize)]
pub(crate) struct Request<'a> {
    pub request_line: RequestLine<'a>,
    pub request_header: RequestHeader<'a>,
    pub request_body: Option<String>,
}

impl<'a> Request<'a> {
    pub fn new(request: &'a str) -> Result<Self, &str> {
        let (request_line, request_header, request_body) =
            Self::parse_request(request)?;

        Ok(Self {
            request_line,
            request_header,
            request_body,
        })
    }

    fn parse_request(
        request: &'a str,
    ) -> Result<(RequestLine<'a>, RequestHeader<'a>, Option<String>), &str>
    {
        let mut lines = request.lines();

        let request_line_str = lines.next().ok_or("empty request.")?;
        let request_line = RequestLine::new(request_line_str)?;

        let mut request_header_lines = Vec::new();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            request_header_lines.push(line);
        }
        let request_header = RequestHeader::new(request_header_lines)?;

        if request_header.content_length.is_none() {
            Ok((request_line, request_header, None))
        } else {
            let request_body = lines.collect::<Vec<&str>>().join("\n");
            Ok((request_line, request_header, Some(request_body)))
        }
    }
}
