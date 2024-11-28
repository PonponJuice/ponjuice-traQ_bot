use std::{env, net::SocketAddr};

use axum::{body::Bytes, extract::State, routing::post, Router};
use http::{HeaderMap, StatusCode};
use tokio::net::TcpListener;
use traq::apis::configuration::Configuration;
use traq_bot_http::{Event, RequestParser};

#[derive(Clone)]
struct App {
    request_parser: RequestParser,
    client_config: Configuration,
}

#[tokio::main]
async fn main() {
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
    let router = Router::new().route("/", post(handler)).with_state(app);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let server = TcpListener::bind(addr).await.unwrap();
    axum::serve(server, router).await.unwrap();
}

async fn handler(State(app): State<App>, headers: HeaderMap, body: Bytes) -> StatusCode {
    match app.request_parser.parse(headers.iter(), &body) {
        Ok(Event::MessageCreated(payload)) => {
            use traq::apis::message_api::post_message;
            print!(
                "{}さんがメッセージを投稿しました。\n内容: {}\n",
                payload.message.user.display_name, payload.message.text
            );

            let request = traq::models::PostMessageRequest {
                content: format!(":oisu-: {}", payload.message.user.display_name),
                embed: None,
            };

            let res = post_message(&app.client_config, &payload.message.channel_id, Some(request)).await;

            if let Err(e) = res {
                eprintln!("{e}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }

            StatusCode::NO_CONTENT
        }
        Ok(Event::DirectMessageCreated(payload)) => {
            use traq::apis::message_api::post_direct_message;
            let user = payload.message.user;
            
            print!(
                "{}さんがメッセージを投稿しました。\n内容: {}\n",
                user.display_name, payload.message.text
            );

            let request = traq::models::PostMessageRequest {
                content: ":oisu-:".to_string(),
                embed: None,
            };
            let res = post_direct_message(&app.client_config, &user.id, Some(request)).await;
            if let Err(e) = res {
                eprintln!("{e}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            StatusCode::NO_CONTENT
        }
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            eprintln!("ERROR: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
