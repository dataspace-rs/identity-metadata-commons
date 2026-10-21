//! Model shared by the [identity metadata service](https://github.com/dataspace-rs/identity-metadata-service)
//! and its client: the JSON-LD identity metadata of organizations (schema.org `Organization`),
//! their trusted issuers, and the problem details of errors.
//!
//! With the `utoipa` feature, the types implement `utoipa::ToSchema`.

mod issuer;
mod organization;
mod problem;

pub use issuer::{NewIssuer, TrustedIssuer, ValidationMethod};
pub use organization::{
  Address, Location, OrganizationDocument, OrganizationGraph, OrganizationType, Place, PlaceType,
  PostalAddress, PostalAddressType, SCHEMA_ORG_CONTEXT,
};
pub use problem::ProblemDetails;
