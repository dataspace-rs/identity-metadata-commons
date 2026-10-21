# Identity Metadata Commons

Model shared by the [identity metadata service](https://github.com/dataspace-rs/identity-metadata-service)
and its [client](https://github.com/dataspace-rs/identity-metadata-client):

- **Organizations**: the identity metadata of an organization, a schema.org
  [`Organization`](https://schema.org/Organization) as a JSON-LD document compacted with the
  `https://schema.org` context (`OrganizationDocument`, `OrganizationGraph`, `Location`, `Place`,
  `Address`, `PostalAddress`).
- **Trusted issuers**: the OpenID providers whose tokens may update an organization, validated by
  token introspection (RFC 7662) or by JWT verification with the provider's `jwks_uri`
  (`NewIssuer`, `TrustedIssuer`, `ValidationMethod`).
- **Identifiers**: the identifier of an organization is an OASIS ebCore Party Id built on
  ISO 6523, `urn:oasis:names:tc:ebcore:partyid-type:iso6523:<ICD>:<organization id>`, with
  check-digit validation of SIRENE, SIRET, DUNS and GLN identifiers (`PartyId`, `PartyIdError`).
- **Errors**: RFC 9457 problem details, `application/problem+json` (`ProblemDetails`).

## Usage

```toml
[dependencies]
identity-metadata-commons = "0.2.0"
```

```rust
use identity_metadata_commons::{NewIssuer, OrganizationDocument};

let organization = OrganizationDocument::new(
  // `parse` fails on an invalid identifier, e.g. a SIRET with a wrong check digit.
  "urn:oasis:names:tc:ebcore:partyid-type:iso6523:0009:55210055400013".parse()?,
  "Example",
)
.with_legal_name("Example SAS")
.with_location("Paris, France")
.with_logo("https://example.com/logo.png");

let json = serde_json::to_string_pretty(&organization)?;

let issuer = NewIssuer::jwks("https://auth.example.com/realms/example").with_audience("identity-metadata");
```

`json` is:

```json
{
  "@context": "https://schema.org",
  "@type": "Organization",
  "identifier": "urn:oasis:names:tc:ebcore:partyid-type:iso6523:0009:55210055400013",
  "name": "Example",
  "legalName": "Example SAS",
  "location": "Paris, France",
  "logo": "https://example.com/logo.png"
}
```

Unknown properties are rejected on deserialization.

## Features

- `utoipa`: the types implement `utoipa::ToSchema`, for OpenAPI documents.

## Minimum supported Rust version

Rust 1.89.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
