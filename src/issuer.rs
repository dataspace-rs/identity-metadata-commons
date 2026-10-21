//! Trusted issuers of the organizations: the OpenID providers whose tokens may update them.

use serde::{Deserialize, Serialize};

/// An OpenID provider to trust.
///
/// With `clientId` and `clientSecret` (a client of the service at the provider), tokens are
/// validated by introspection: the discovery document must expose an `introspection_endpoint`.
/// Without them, JWT access tokens are verified with the keys published at the provider's
/// `jwks_uri`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NewIssuer {
  /// OpenID issuer URL, base of the discovery document.
  #[cfg_attr(
    feature = "utoipa",
    schema(example = "https://login.example.com/realms/acme")
  )]
  pub issuer: String,
  /// Audience the tokens must be intended for (`aud` claim); not checked when absent.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "utoipa", schema(example = "identity-metadata-service"))]
  pub audience: Option<String>,
  /// Client used to introspect the issuer's tokens; requires `clientSecret`.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "utoipa", schema(example = "identity-metadata-service"))]
  pub client_id: Option<String>,
  /// Secret of the client; stored, never returned.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "utoipa", schema(write_only))]
  pub client_secret: Option<String>,
}

impl NewIssuer {
  /// A provider whose JWT access tokens are verified with its published keys.
  pub fn jwks(issuer: impl Into<String>) -> Self {
    Self {
      issuer: issuer.into(),
      audience: None,
      client_id: None,
      client_secret: None,
    }
  }

  /// A provider whose tokens are introspected with a client of the service at the provider.
  pub fn introspection(
    issuer: impl Into<String>,
    client_id: impl Into<String>,
    client_secret: impl Into<String>,
  ) -> Self {
    Self {
      client_id: Some(client_id.into()),
      client_secret: Some(client_secret.into()),
      ..Self::jwks(issuer)
    }
  }

  pub fn with_audience(mut self, audience: impl Into<String>) -> Self {
    self.audience = Some(audience.into());
    self
  }
}

/// How the tokens of a trusted issuer are validated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum ValidationMethod {
  /// Token introspection (RFC 7662) with the OAuth 2.0 client of the service at the issuer.
  Introspection,
  /// Signature verification of JWT access tokens with the keys at the issuer's `jwks_uri`.
  Jwks,
}

/// An OpenID provider whose users may update an organization, with the OAuth 2.0 client
/// registered at that provider for the service when its tokens are introspected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TrustedIssuer {
  /// Identifier of the registration, assigned by the server.
  #[cfg_attr(
    feature = "utoipa",
    schema(example = "0b6c4f4e-8f0a-4c43-9d8e-2f4a3b1c5d6e")
  )]
  pub id: String,
  /// OpenID issuer URL, base of the discovery document.
  #[cfg_attr(
    feature = "utoipa",
    schema(example = "https://login.example.com/realms/acme")
  )]
  pub issuer: String,
  /// How the tokens of the issuer are validated.
  pub token_validation: ValidationMethod,
  /// Audience the tokens must be intended for (`aud` claim); not checked when absent.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "utoipa", schema(example = "identity-metadata-service"))]
  pub audience: Option<String>,
  /// OAuth 2.0 client of the service at the issuer, used for token introspection.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "utoipa", schema(example = "identity-metadata-service"))]
  pub client_id: Option<String>,
}
