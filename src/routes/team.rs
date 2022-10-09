pub mod v2 {
    use axum::http::StatusCode;

  pub async fn team() -> Result<(), StatusCode>{
    Err(StatusCode::NOT_IMPLEMENTED)
  }
  pub async fn teams() -> Result<(), StatusCode>{
    Err(StatusCode::NOT_IMPLEMENTED)
  }
}