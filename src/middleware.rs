use axum::{
  extract::FromRequestParts,
  http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use helpers::jwt;

use crate::AppState;

pub struct RequireAuth {
  pub user_id: i32,
}

impl FromRequestParts<AppState> for RequireAuth {
  type Rejection = StatusCode;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &AppState,
  ) -> Result<Self, Self::Rejection> {
    let auth_header = parts
      .headers
      .get(AUTHORIZATION)
      .and_then(|value| value.to_str().ok());

    if let Some(auth_header) = auth_header
      && auth_header.starts_with("Bearer ")
    {
      let token = auth_header.trim_start_matches("Bearer ");
      if let Ok(data) = jwt::verify::<i32>(token, &state.jwt_key) {
        Ok(RequireAuth {
          user_id: data.claims.data,
        })
      } else {
        Err(StatusCode::UNAUTHORIZED)
      }
    } else {
      Err(StatusCode::UNAUTHORIZED)
    }
  }
}
