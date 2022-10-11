use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension, Json,
};
use tracing::instrument;

use crate::app::App;

#[instrument]
pub async fn editlike_handler(
    Json(payload): Json<shared::EditLike>,
    Path(id): Path<String>,
    Extension(app): Extension<App>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("edit like: {}/{}", payload.question_id, id);

    match app.edit_like(id, payload).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[instrument]
pub async fn addevent_handler(
    Json(payload): Json<shared::AddEvent>,
    Extension(app): Extension<App>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("create event: {}", payload.data.name);

    match app.create_event(payload).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[instrument]
pub async fn addquestion_handler(
    Json(payload): Json<shared::AddQuestion>,
    Path(id): Path<String>,
    Extension(app): Extension<App>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("add question: {} in event:  {}", payload.text, id);

    match app.add_question(id, payload).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[instrument]
pub async fn getevent_handler(
    Path(id): Path<String>,
    Extension(app): Extension<App>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("get event:  {}", id);

    match app.get_event(id, None).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[instrument]
pub async fn get_modevent_handler(
    Path((id, secret)): Path<(String, String)>,
    Extension(app): Extension<App>,
) -> Result<impl IntoResponse, StatusCode> {
    match app.get_event(id, Some(secret)).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[instrument]
pub async fn ping_handler() -> Html<&'static str> {
    Html("pong")
}