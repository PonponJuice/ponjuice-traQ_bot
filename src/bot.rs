use axum::{body::Bytes, extract::State};
use http::{HeaderMap, StatusCode};
use traq_bot_http::Event;

use crate::App;
mod message;
mod util;

pub async fn bot_handle(State(app): State<App>, headers: HeaderMap, body: Bytes) -> StatusCode {
    let event = match app.request_parser.parse(headers.iter(), &body) {
        Ok(event) => event,
        Err(err) => {
            eprintln!("ERROR: {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    match event {
        Event::MessageCreated(payload) => message::message_created(app, payload).await,
        Event::DirectMessageCreated(payload) => message::direct_message_created(app, payload).await,
        _ => StatusCode::NO_CONTENT,
    }
}
