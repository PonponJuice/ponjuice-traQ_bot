use std::{env, net::SocketAddr};

use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;
use traq::apis::configuration::Configuration;
use traq_bot_http::RequestParser;

mod bot;
mod handler;

#[derive(Clone)]
struct App {
    request_parser: RequestParser,
    client_config: Configuration,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("info".into()))
        .init();

    let verification_token = env::var("VERIFICATION_TOKEN").unwrap();
    let access_token = env::var("BOT_ACCESS_TOKEN").unwrap();
    let request_parser = RequestParser::new(&verification_token);
    let client_config = Configuration {
        bearer_access_token: Some(access_token),
        ..Default::default()
    };
    let app = App {
        request_parser,
        client_config,
    };

    let router = handler::make_router(app);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let server = TcpListener::bind(addr).await.unwrap();
    axum::serve(server, router).await.unwrap();
}
