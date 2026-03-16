use axum::{
  body::Body,
  http::{HeaderMap, Response, StatusCode, header},
  response::IntoResponse,
};

pub async fn handle_client_js(headers: HeaderMap) -> impl IntoResponse {
  let js = include_str!("../../ui/dist/client/client.js");

  #[cfg(debug_assertions)]
  let (etag, cache_ctrl): (Option<&str>, &str) = (None, "no-store, must-revalidate");

  #[cfg(not(debug_assertions))]
  let (etag, cache_ctrl) = (
    Some(concat!("\"", env!("CARGO_PKG_VERSION"), "\"")),
    "no-cache",
  );

  if let Some(current_etag) = etag
    && let Some(if_none_match) = headers.get(header::IF_NONE_MATCH)
    && if_none_match == current_etag
  {
    return StatusCode::NOT_MODIFIED.into_response();
  }

  let mut builder = Response::builder()
    .header(header::CONTENT_TYPE, "application/javascript")
    .header(header::CACHE_CONTROL, cache_ctrl);

  if let Some(current_etag) = etag {
    builder = builder.header(header::ETAG, current_etag);
  }

  builder.body(Body::from(js)).unwrap().into_response()
}

pub async fn handle_admin_js(headers: HeaderMap) -> impl IntoResponse {
  let js = include_str!("../../ui/dist/admin/admin.js");

  #[cfg(debug_assertions)]
  let (etag, cache_ctrl): (Option<&str>, &str) = (None, "no-store, must-revalidate");

  #[cfg(not(debug_assertions))]
  let (etag, cache_ctrl) = (
    Some(concat!("\"", env!("CARGO_PKG_VERSION"), "\"")),
    "no-cache",
  );

  if let Some(current_etag) = etag
    && let Some(if_none_match) = headers.get(header::IF_NONE_MATCH)
    && if_none_match == current_etag
  {
    return StatusCode::NOT_MODIFIED.into_response();
  }

  let mut builder = Response::builder()
    .header(header::CONTENT_TYPE, "application/javascript")
    .header(header::CACHE_CONTROL, cache_ctrl);

  if let Some(current_etag) = etag {
    builder = builder.header(header::ETAG, current_etag);
  }

  builder.body(Body::from(js)).unwrap().into_response()
}
