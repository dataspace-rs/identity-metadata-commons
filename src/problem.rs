use serde::{Deserialize, Serialize};

/// RFC 9457 problem details (`application/problem+json`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", schema(example = json!({
  "type": "about:blank",
  "title": "Not Found",
  "status": 404,
  "detail": "organization `6f1c2b9e-3d4a-4e8b-9c1f-2a7d5e8b4c3a` not found"
})))]
pub struct ProblemDetails {
  #[serde(rename = "type")]
  pub kind: String,
  pub title: String,
  pub status: u16,
  pub detail: String,
}
