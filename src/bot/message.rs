use std::path::PathBuf;

use http::StatusCode;
use svg::node::element::{path::Data, Path};
use traq::{apis::{configuration, file_api::PostFileError, Error, ResponseContent}, models};
use traq_bot_http::payloads::{DirectMessageCreatedPayload, MessageCreatedPayload};

use crate::App;

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
        "{}さんがメッセージを投稿しました。\n内容: {}\n{}",
        user.display_name,
        payload.message.text,
        payload.message.channel_id
    );

    let file = make_svg_file();

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

fn make_svg_file() -> PathBuf {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 2)
        .set("d", data);

    let document = svg::Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(path);

    svg::save("./image.svg", &document).unwrap();

    PathBuf::from("./image.svg")
}

/// 指定したチャンネルにファイルをアップロードします。 アーカイブされているチャンネルにはアップロード出来ません。
pub async fn post_file(
    configuration: &configuration::Configuration,
    file: std::path::PathBuf,
    channel_id: &str,
) -> Result<models::FileInfo, Error<PostFileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'file' parameter
    let data = std::fs::read(file)?;
    let filedata = reqwest::multipart::Part::bytes(data).file_name("image.svg");

    local_var_form = local_var_form.text("channelId", channel_id.to_string());
    local_var_form = local_var_form.part("file", filedata);

    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostFileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
