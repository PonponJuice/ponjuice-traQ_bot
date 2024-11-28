use http::StatusCode;
use traq_bot_http::payloads::{DirectMessageCreatedPayload, MessageCreatedPayload};

use crate::{
    bot::util::{make_svg_file, post_file},
    App,
};

pub async fn message_created(app: App, payload: MessageCreatedPayload) -> StatusCode {
    use traq::apis::message_api::post_message;
    tracing::info!(
        "{}さんがメッセージを投稿しました。\n内容: {}",
        payload.message.user.display_name,
        payload.message.text
    );

    let request = traq::models::PostMessageRequest {
        content: format!(":oisu-: {}", payload.message.user.display_name),
        embed: None,
    };

    let res = post_message(
        &app.client_config,
        &payload.message.channel_id,
        Some(request),
    )
    .await;

    if let Err(e) = res {
        tracing::error!("{e}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::NO_CONTENT
}

pub async fn direct_message_created(app: App, payload: DirectMessageCreatedPayload) -> StatusCode {
    use traq::apis::message_api::post_direct_message;
    let user = payload.message.user;

    tracing::info!(
        "{}さんがメッセージを投稿しました。\n内容: {}",
        user.display_name,
        payload.message.text
    );

    let file_name = "image.svg";
    let file = make_svg_file(file_name);

    let resp = post_file(&app.client_config, file, &payload.message.channel_id).await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => {
            tracing::error!("{e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let request = traq::models::PostMessageRequest {
        content: format!("これはテストです\n\nhttps://q.trap.jp/files/{}", resp.id),
        embed: None,
    };
    let res = post_direct_message(&app.client_config, &user.id, Some(request)).await;
    if let Err(e) = res {
        tracing::error!("{e}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::NO_CONTENT
}
