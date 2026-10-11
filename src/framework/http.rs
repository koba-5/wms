use crate::framework::http::config::Config;
use crate::framework::http::request::Request;
use crate::framework::logger::Logger;
use serde::Serialize;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod config;
mod request;

pub struct Http {
    config: Config,
    logger: Logger,
}

impl Http {
    pub fn new() -> Self {
        let config = Config::new();
        let logger = Logger::new();

        Self { config, logger }
    }

    pub fn start(&self) {
        let address = &format!("{}:{}", self.config.ip, self.config.port);
        let listener = TcpListener::bind(address).unwrap();
        self.logger.info(&format!("Listening on {}.", address));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self
                    .handle_client(stream)
                    .expect("error while handling client."),
                Err(e) => {
                    self.logger.error(&format!("Connection failed: {}.", e))
                }
            }
        }
    }

    fn handle_client(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer)?;

        let request_raw = String::from_utf8_lossy(&buffer[..n]);

        let response = match Request::new(&request_raw) {
            Ok(request) => {
                self.logger.info(&serde_json::to_string(&request)?);
                "HTTP/1.1 200 OK\r\n\
                Content-Type: application/json; charset=UTF-8\r\n\
                \r\n\
                {\"message\": \"Hello World\"}"
            }
            Err(e) => {
                self.logger.info(&format!("Bad Request: {}", e));
                "HTTP/1.1 400 Bad Request\r\n\
                Content-Type: application/json; charset=UTF-8\r\n\
                \r\n\
                {\"message\": \"Bad Request\"}"
            }
        };

        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
