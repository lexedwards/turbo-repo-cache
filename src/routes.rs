use axum::http::StatusCode;
pub mod artifacts;
pub mod team;
pub mod user;
pub async fn status () -> Result<StatusCode, ()> {
  Ok(StatusCode::OK)
}