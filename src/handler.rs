use axum::routing::post;
use axum::Router;

use crate::bot::bot_handle;
use crate::App;

pub fn make_router(app: App) -> Router {
    Router::new()
        .route("/", post(bot_handle))
        .with_state(app)
}
