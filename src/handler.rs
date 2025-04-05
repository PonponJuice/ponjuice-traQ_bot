use axum::routing::{get, post};
use axum::Router;

use crate::bot::bot_handle;
use crate::App;

async fn test_handler() -> &'static str {
    "Hello, world!"
}

async fn get_atcoder() -> Result<String, String> {
    let url = "https://atcoder.jp/users/ponjuice/history/json";

    let response = reqwest::get(url).await.map_err(|e| {tracing::info!("{:?}" ,e); e.to_string()})?;

    tracing::info!("Response Status: {:?}", response.status());
    tracing::info!("Response Content-Length: {:?}", response.content_length());
    tracing::info!("Response Headers: {:?}", response.headers());
    tracing::info!("Response URL: {:?}", response.url());
    tracing::info!("Response Version: {:?}", response.version());
    tracing::info!("Response Text: {:?}", response.text().await.map_err(|e| {tracing::info!("{:?}" ,e); e.to_string()})?);

    Ok("ok".to_string())
}

pub fn make_router(app: App) -> Router {
    Router::new()
        .route("/", post(bot_handle))
        .route("/test", get(test_handler))
        .route("/get", get(get_atcoder))
        .with_state(app)
}
