pub mod v8 {
    use std::io;
    use axum::{
      Json,
      body::StreamBody,
      http::StatusCode,
      response::IntoResponse,
      extract::{Query,Path,BodyStream},
    };
    use futures::{
      TryStreamExt
    };
    use tokio::{fs::File, io::BufWriter};
    use tokio_util::io::{ReaderStream, StreamReader};
    use serde::Deserialize;
    use serde_json::json;
    use log::debug;

    #[derive(Debug, Deserialize)]
    #[allow(non_snake_case, dead_code)]
    pub struct Params {
      teamId: Option<String>,
      slug: Option<String>
    }

  pub const CACHE_DIR: &str = ".cache";

  fn parse_team_id(params: &Params) -> Result<&String, (StatusCode, String)> {
    params.teamId.as_ref().or(params.slug.as_ref()).ok_or_else(|| (StatusCode::NOT_FOUND, format!("Could not find Team ID")))
  }

  pub async fn get(
    Path(artifact_id): Path<String>,
    Query(params): Query<Params>
  ) -> Result<StreamBody<ReaderStream<File>>, (StatusCode, std::string::String)> {
    let team_id = parse_team_id(&params)?;
    debug!("[GET] id: {}, teamId: {:?}",artifact_id, team_id);
    let file_path = std::path::Path::new(CACHE_DIR).join(&team_id).join(&artifact_id);
    debug!("Expected File to be at {:?}", file_path);
    let file = match File::open(std::path::Path::new(CACHE_DIR).join(&team_id).join(&artifact_id)).await {
      Ok(file) => file,
      Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err.to_string())))
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    Ok(body)
  }

  pub async fn put(
    Path(artifact_id): Path<String>,
    Query(params): Query<Params>,
    body: BodyStream
  ) -> impl IntoResponse {
    let team_id = parse_team_id(&params)?;
    debug!("[PUT] id: {}, params: {:?}",artifact_id, team_id);

    async {
      // Convert to AsyncRead.
      let body_with_io_error = body.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
      let body_reader = StreamReader::new(body_with_io_error);
      futures::pin_mut!(body_reader);

      let path = std::path::Path::new(CACHE_DIR).join(&team_id);
      std::fs::create_dir_all(&path);
      let mut file = BufWriter::new(File::create(path.join(&artifact_id)).await?);

      tokio::io::copy(&mut body_reader, &mut file).await?;

      let response_json = json!({
        "urls": vec![format!("{}/{:?}", team_id,artifact_id)]
      });

      Ok::<_,io::Error>((
        StatusCode::OK,
        Json(response_json)
      ))
    }.await.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
  }

  pub async fn events() -> Result<(), StatusCode>{
    Err(StatusCode::NOT_IMPLEMENTED)
  }

  pub async fn status() -> Result<(), StatusCode>{
    Err(StatusCode::NOT_IMPLEMENTED)
  }
}