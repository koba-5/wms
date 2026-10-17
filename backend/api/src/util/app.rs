use crate::util::config::from_env;
use crate::util::logger::Logger;
use axum::Router;
use std::net::SocketAddr;

pub struct App {
    addr: SocketAddr,
}

impl App {
    pub fn new() -> Self {
        let ip = from_env("SERVER_IP");
        let port = from_env("SERVER_PORT").parse::<u16>().unwrap();
        let addr = format!("{}:{}", ip, port)
            .parse::<SocketAddr>()
            .expect("Invalid SERVER_IP or SERVER_PORT");

        Self { addr }
    }

    pub async fn execute<F>(&self, f: F)
    where
        F: AsyncFnOnce(Router) -> Router,
    {
        let router_setup = Router::new();
        let router = f(router_setup).await;

        let listener = tokio::net::TcpListener::bind(self.addr).await.unwrap();
        Logger::info("Listening on http://127.0.0.1:3000");
        axum::serve(listener, router).await.unwrap();
    }
}
