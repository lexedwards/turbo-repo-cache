pub mod v2 {
    use axum::http::StatusCode;

  pub async fn user() -> Result<(), StatusCode>{
    Err(StatusCode::NOT_IMPLEMENTED)
  }
}