//! JSON-LD representation of the identity metadata, based on schema.org `Organization`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::PartyId;

/// The JSON-LD context of the documents.
pub const SCHEMA_ORG_CONTEXT: &str = "https://schema.org";

/// Single-valued `@type` marker types, so that serde only accepts the expected type name.
macro_rules! type_marker {
  ($name:ident, $value:ident) => {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
    pub enum $name {
      #[default]
      $value,
    }
  };
}
type_marker!(OrganizationType, Organization);
type_marker!(PlaceType, Place);
type_marker!(PostalAddressType, PostalAddress);

/// Identity metadata of an organization: a schema.org `Organization` as a JSON-LD document
/// compacted with the schema.org context. Unknown properties are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
  feature = "utoipa",
  schema(
    as = Organization,
    example = json!({
      "@context": "https://schema.org",
      "@type": "Organization",
      "identifier": "urn:oasis:names:tc:ebcore:partyid-type:iso6523:0002:552100554",
      "name": "ACME",
      "legalName": "ACME SAS",
      "logo": "https://acme.example/logo.png",
      "location": {
        "@type": "Place",
        "name": "Head office",
        "address": {"@type": "PostalAddress", "addressLocality": "Paris", "addressCountry": "FR"}
      }
    })
  )
)]
pub struct OrganizationDocument {
  /// JSON-LD context: `https://schema.org` (required on input; omitted in `@graph` items).
  #[serde(rename = "@context", default, skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "utoipa", schema(value_type = Option<String>, example = "https://schema.org"))]
  pub context: Option<Value>,
  /// IRI of the record, `<base>/organizations/<id>` where `id` is the UUID assigned by the
  /// server at registration; ignored on input.
  #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "utoipa", schema(read_only))]
  pub id: Option<String>,
  #[serde(rename = "@type")]
  pub kind: OrganizationType,
  /// OASIS ebCore Party Id built on ISO 6523:
  /// `urn:oasis:names:tc:ebcore:partyid-type:iso6523:<ICD>:<organization id>`.
  /// Unique among the organizations.
  #[cfg_attr(
    feature = "utoipa",
    schema(example = "urn:oasis:names:tc:ebcore:partyid-type:iso6523:0002:552100554")
  )]
  pub identifier: String,
  /// Common name of the organization.
  pub name: String,
  /// Official name of the organization.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub legal_name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub location: Option<Location>,
  /// Absolute http(s) URL of the logo.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "utoipa", schema(format = "uri"))]
  pub logo: Option<String>,
}

impl OrganizationDocument {
  /// A new document in the schema.org context, with the required properties; the identifier is
  /// validated by parsing it into a [`PartyId`].
  pub fn new(identifier: PartyId, name: impl Into<String>) -> Self {
    Self {
      context: Some(Value::String(SCHEMA_ORG_CONTEXT.to_owned())),
      id: None,
      kind: OrganizationType::Organization,
      identifier: identifier.to_string(),
      name: name.into(),
      legal_name: None,
      location: None,
      logo: None,
    }
  }

  pub fn with_legal_name(mut self, legal_name: impl Into<String>) -> Self {
    self.legal_name = Some(legal_name.into());
    self
  }

  pub fn with_location(mut self, location: impl Into<Location>) -> Self {
    self.location = Some(location.into());
    self
  }

  pub fn with_logo(mut self, logo: impl Into<String>) -> Self {
    self.logo = Some(logo.into());
    self
  }

  /// The id of the organization, the last segment of its `@id` IRI.
  pub fn organization_id(&self) -> Option<Uuid> {
    let id = self.id.as_deref()?;
    Uuid::parse_str(id.trim_end_matches('/').rsplit('/').next()?).ok()
  }
}

/// schema.org `location`: `Text`, `Place` or `PostalAddress`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(untagged)]
pub enum Location {
  Text(String),
  Place(Place),
  PostalAddress(PostalAddress),
}

impl From<String> for Location {
  fn from(text: String) -> Self {
    Self::Text(text)
  }
}

impl From<&str> for Location {
  fn from(text: &str) -> Self {
    Self::Text(text.to_owned())
  }
}

impl From<Place> for Location {
  fn from(place: Place) -> Self {
    Self::Place(place)
  }
}

impl From<PostalAddress> for Location {
  fn from(address: PostalAddress) -> Self {
    Self::PostalAddress(address)
  }
}

/// schema.org `Place`, with a `name` and/or an `address`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Place {
  #[serde(rename = "@type")]
  pub kind: PlaceType,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub address: Option<Address>,
}

/// schema.org `address`: `Text` or `PostalAddress`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(untagged)]
pub enum Address {
  Text(String),
  PostalAddress(PostalAddress),
}

impl From<String> for Address {
  fn from(text: String) -> Self {
    Self::Text(text)
  }
}

impl From<&str> for Address {
  fn from(text: &str) -> Self {
    Self::Text(text.to_owned())
  }
}

impl From<PostalAddress> for Address {
  fn from(address: PostalAddress) -> Self {
    Self::PostalAddress(address)
  }
}

/// schema.org `PostalAddress`, with at least one property.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PostalAddress {
  #[serde(rename = "@type")]
  pub kind: PostalAddressType,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub street_address: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub post_office_box_number: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub postal_code: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub address_locality: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub address_region: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub address_country: Option<String>,
}

impl PostalAddress {
  /// The properties of the address, by schema.org name.
  pub fn fields(&self) -> [(&'static str, &Option<String>); 6] {
    [
      ("streetAddress", &self.street_address),
      ("postOfficeBoxNumber", &self.post_office_box_number),
      ("postalCode", &self.postal_code),
      ("addressLocality", &self.address_locality),
      ("addressRegion", &self.address_region),
      ("addressCountry", &self.address_country),
    ]
  }

  /// The property of the address with the schema.org name `name`.
  pub fn field_mut(&mut self, name: &str) -> Option<&mut Option<String>> {
    Some(match name {
      "streetAddress" => &mut self.street_address,
      "postOfficeBoxNumber" => &mut self.post_office_box_number,
      "postalCode" => &mut self.postal_code,
      "addressLocality" => &mut self.address_locality,
      "addressRegion" => &mut self.address_region,
      "addressCountry" => &mut self.address_country,
      _ => return None,
    })
  }
}

/// The registered organizations, as a JSON-LD graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct OrganizationGraph {
  #[serde(rename = "@context")]
  #[cfg_attr(feature = "utoipa", schema(example = "https://schema.org"))]
  pub context: String,
  /// The organizations, without `@context`.
  #[serde(rename = "@graph")]
  pub graph: Vec<OrganizationDocument>,
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn serializes_organization() {
    let org = OrganizationDocument::new(
      "urn:oasis:names:tc:ebcore:partyid-type:iso6523:0002:552100554"
        .parse()
        .unwrap(),
      "ACME",
    )
    .with_legal_name("ACME SAS")
    .with_location(Place {
      name: Some("Head office".into()),
      address: Some(
        PostalAddress {
          address_locality: Some("Paris".into()),
          address_country: Some("FR".into()),
          ..Default::default()
        }
        .into(),
      ),
      ..Default::default()
    });
    assert_eq!(
      serde_json::to_value(&org).unwrap(),
      json!({
        "@context": "https://schema.org",
        "@type": "Organization",
        "identifier": "urn:oasis:names:tc:ebcore:partyid-type:iso6523:0002:552100554",
        "name": "ACME",
        "legalName": "ACME SAS",
        "location": {
          "@type": "Place",
          "name": "Head office",
          "address": {"@type": "PostalAddress", "addressLocality": "Paris", "addressCountry": "FR"}
        }
      })
    );
  }

  #[test]
  fn deserializes_locations() {
    let location = |value| serde_json::from_value::<Location>(value).unwrap();
    assert_eq!(location(json!("Paris")), Location::Text("Paris".into()));
    assert!(matches!(
      location(json!({"@type": "PostalAddress", "postalCode": "75001"})),
      Location::PostalAddress(_)
    ));
    assert!(matches!(
      location(json!({"@type": "Place", "address": "Paris"})),
      Location::Place(Place {
        address: Some(Address::Text(_)),
        ..
      })
    ));
    assert!(serde_json::from_value::<Location>(json!({"@type": "City", "name": "Paris"})).is_err());
  }

  #[test]
  fn rejects_unknown_properties() {
    let doc = json!({
      "@context": "https://schema.org",
      "@type": "Organization",
      "identifier": "urn:x",
      "name": "ACME",
      "unknown": "x"
    });
    assert!(serde_json::from_value::<OrganizationDocument>(doc).is_err());
  }

  #[test]
  fn reads_organization_id() {
    let mut org = OrganizationDocument::new(
      "urn:oasis:names:tc:ebcore:partyid-type:iso6523:0002:552100554"
        .parse()
        .unwrap(),
      "ACME",
    );
    assert_eq!(org.organization_id(), None);
    org.id = Some("https://ims.example/organizations/6f1c2b9e-3d4a-4e8b-9c1f-2a7d5e8b4c3a".into());
    assert_eq!(
      org.organization_id(),
      Some(Uuid::parse_str("6f1c2b9e-3d4a-4e8b-9c1f-2a7d5e8b4c3a").unwrap())
    );
  }
}
