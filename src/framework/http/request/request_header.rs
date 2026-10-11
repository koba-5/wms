use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct RequestHeader<'a> {
    user_agent: Option<&'a str>,
    pub content_length: Option<usize>,
}

impl<'a> RequestHeader<'a> {
    pub fn new(request_header_lines: Vec<&'a str>) -> Result<Self, &'a str> {
        let mut user_agent = None;
        let mut content_length = None;

        for line in request_header_lines {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                if key.eq_ignore_ascii_case("User-Agent") {
                    user_agent = Some(value);
                } else if key.eq_ignore_ascii_case("Content-Length") {
                    if let Ok(len) = value.parse::<usize>() {
                        if len > 0 {
                            content_length = Some(len);
                        }
                    }
                }
            }
        }

        Ok(Self {
            user_agent,
            content_length,
        })
    }
}
