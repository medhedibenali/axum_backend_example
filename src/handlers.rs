use crate::{
    app_state::AppState,
    errors::Error,
    models::{CreateUser, User},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_extra::extract::WithRejection;
use std::sync::Arc;
use uuid::Uuid;

type HandlerResponse<T> = (StatusCode, Json<T>);
type HandlerResult<T> = Result<HandlerResponse<T>, Error>;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(
    State(app_state): State<Arc<AppState>>,
    WithRejection(Json(payload), _): WithRejection<Json<CreateUser>, Error>,
) -> HandlerResult<User> {
    let result = app_state
        .client
        .query_one(
            "INSERT INTO \"user\" (first_name, last_name, username) VALUES ($1, $2, $3) RETURNING *",
            &[&payload.first_name, &payload.last_name, &payload.username],
        )
        .await;

    result
        .map(|row| (StatusCode::CREATED, Json(row.into())))
        .map_err(|error| {
            Error::new(
                StatusCode::BAD_REQUEST,
                error.as_db_error().unwrap().detail().unwrap(),
            )
        })
}

pub async fn get_users(State(app_state): State<Arc<AppState>>) -> (StatusCode, Json<Vec<User>>) {
    let users: Vec<User> = app_state
        .client
        .query("SELECT * FROM \"user\"", &[])
        .await
        .unwrap()
        .into_iter()
        .map(|value| value.into())
        .collect();

    (StatusCode::OK, Json(users))
}

pub async fn get_user(
    State(app_state): State<Arc<AppState>>,
    WithRejection(Path(id), _): WithRejection<Path<Uuid>, Error>,
) -> HandlerResult<User> {
    let result = app_state
        .client
        .query_one("SELECT * FROM \"user\" WHERE id=$1", &[&id])
        .await;

    result
        .map(|row| (StatusCode::OK, Json(row.into())))
        .map_err(|error| Error::new(StatusCode::NOT_FOUND, error))
}
