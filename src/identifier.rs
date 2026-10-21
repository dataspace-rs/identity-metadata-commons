//! Validation of OASIS ebCore Party Id Type identifiers built on ISO 6523.
//!
//! The ebCore Party Id Type specification defines URNs of the form
//! `urn:oasis:names:tc:ebcore:partyid-type:iso6523:<ICD>:<organization identifier>`,
//! e.g. `urn:oasis:names:tc:ebcore:partyid-type:iso6523:0002:552100554`, where
//! `<ICD>` is the 4-digit International Code Designator of the ISO 6523 registry.

use std::fmt;
use std::str::FromStr;

/// Prefix of the identifiers: the OASIS ebCore Party Id Type URN for ISO 6523.
pub const ISO6523_PREFIX: &str = "urn:oasis:names:tc:ebcore:partyid-type:iso6523:";

/// ISO 6523 limits the organization identifier to 35 characters.
const MAX_ORGANIZATION_ID_LEN: usize = 35;

/// An ebCore Party Id of an organization, parsed with [`FromStr`] and written with [`Display`](fmt::Display).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartyId {
  icd: String,
  organization_id: String,
}

/// Why an identifier is not a valid [`PartyId`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PartyIdError {
  #[error("identifier must start with `{ISO6523_PREFIX}`")]
  InvalidPrefix,
  #[error("ISO 6523 ICD must be exactly 4 digits, got `{0}`")]
  InvalidIcd(String),
  #[error("missing organization identifier after the ICD")]
  MissingOrganizationId,
  #[error("organization identifier must be at most {MAX_ORGANIZATION_ID_LEN} characters")]
  OrganizationIdTooLong,
  #[error("organization identifier contains invalid character `{0}`")]
  InvalidCharacter(char),
  #[error("invalid {scheme} identifier: {reason}")]
  Scheme {
    scheme: &'static str,
    reason: &'static str,
  },
}

impl PartyId {
  /// The ISO 6523 International Code Designator (e.g. `0002` for SIRENE).
  pub fn icd(&self) -> &str {
    &self.icd
  }

  /// The identifier of the organization within the ICD scheme.
  pub fn organization_id(&self) -> &str {
    &self.organization_id
  }
}

impl FromStr for PartyId {
  type Err = PartyIdError;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    // The URN prefix (NID and the OASIS-assigned part) is case-insensitive;
    // it is normalized to lower case.
    let rest = value
      .get(..ISO6523_PREFIX.len())
      .filter(|prefix| prefix.eq_ignore_ascii_case(ISO6523_PREFIX))
      .map(|_| &value[ISO6523_PREFIX.len()..])
      .ok_or(PartyIdError::InvalidPrefix)?;

    let (icd, organization_id) = rest
      .split_once(':')
      .ok_or(PartyIdError::MissingOrganizationId)?;

    if icd.len() != 4 || !icd.bytes().all(|b| b.is_ascii_digit()) {
      return Err(PartyIdError::InvalidIcd(icd.to_owned()));
    }
    if organization_id.is_empty() {
      return Err(PartyIdError::MissingOrganizationId);
    }
    if organization_id.chars().count() > MAX_ORGANIZATION_ID_LEN {
      return Err(PartyIdError::OrganizationIdTooLong);
    }
    if let Some(c) = organization_id
      .chars()
      .find(|c| !(c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_' | '~')))
    {
      return Err(PartyIdError::InvalidCharacter(c));
    }

    validate_scheme(icd, organization_id)?;

    Ok(Self {
      icd: icd.to_owned(),
      organization_id: organization_id.to_owned(),
    })
  }
}

impl fmt::Display for PartyId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{ISO6523_PREFIX}{}:{}", self.icd, self.organization_id)
  }
}

/// Additional checks for well-known ICD schemes. Unknown schemes only get the
/// generic ISO 6523 checks.
fn validate_scheme(icd: &str, id: &str) -> Result<(), PartyIdError> {
  let all_digits = id.bytes().all(|b| b.is_ascii_digit());
  let fail = |scheme, reason| Err(PartyIdError::Scheme { scheme, reason });
  match icd {
    // SIRENE: SIREN (9 digits) or SIRET (14 digits).
    "0002" => match id.len() {
      9 if all_digits && luhn(id) => Ok(()),
      14 if all_digits && siret_checksum(id) => Ok(()),
      9 | 14 if all_digits => fail("SIRENE", "invalid check digit"),
      _ => fail("SIRENE", "expected a 9-digit SIREN or a 14-digit SIRET"),
    },
    "0009" => match id.len() {
      14 if all_digits && siret_checksum(id) => Ok(()),
      14 if all_digits => fail("SIRET", "invalid check digit"),
      _ => fail("SIRET", "expected 14 digits"),
    },
    "0060" if !(id.len() == 9 && all_digits) => fail("DUNS", "expected 9 digits"),
    "0088" => match id.len() {
      13 if all_digits && gs1_check_digit(id) => Ok(()),
      13 if all_digits => fail("GLN", "invalid check digit"),
      _ => fail("GLN", "expected 13 digits"),
    },
    _ => Ok(()),
  }
}

fn digits(id: &str) -> impl DoubleEndedIterator<Item = u32> + '_ {
  id.bytes().map(|b| u32::from(b - b'0'))
}

fn luhn(id: &str) -> bool {
  let sum: u32 = digits(id)
    .rev()
    .enumerate()
    .map(|(i, d)| match (i % 2 == 1, d * 2) {
      (true, doubled) if doubled > 9 => doubled - 9,
      (true, doubled) => doubled,
      (false, _) => d,
    })
    .sum();
  sum.is_multiple_of(10)
}

fn siret_checksum(id: &str) -> bool {
  // La Poste establishments (SIREN 356000000) use a digit sum modulo 5 instead of Luhn.
  luhn(id) || (id.starts_with("356000000") && digits(id).sum::<u32>() % 5 == 0)
}

fn gs1_check_digit(id: &str) -> bool {
  let (body, check) = id.split_at(id.len() - 1);
  let sum: u32 = digits(body)
    .rev()
    .enumerate()
    .map(|(i, d)| if i % 2 == 0 { d * 3 } else { d })
    .sum();
  (10 - sum % 10) % 10 == digits(check).next().unwrap_or(10)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn parse(value: &str) -> Result<PartyId, PartyIdError> {
    value.parse()
  }

  #[test]
  fn accepts_specification_example() {
    let id = parse("urn:oasis:names:tc:ebcore:partyid-type:iso6523:0002:552100554").unwrap();
    assert_eq!(id.icd(), "0002");
    assert_eq!(id.organization_id(), "552100554");
    assert_eq!(
      id.to_string(),
      "urn:oasis:names:tc:ebcore:partyid-type:iso6523:0002:552100554"
    );
  }

  #[test]
  fn normalizes_prefix_case() {
    let id = parse("URN:OASIS:names:tc:ebcore:partyid-type:ISO6523:0088:4035811991021").unwrap();
    assert_eq!(
      id.to_string(),
      "urn:oasis:names:tc:ebcore:partyid-type:iso6523:0088:4035811991021"
    );
  }

  #[test]
  fn accepts_unknown_scheme_with_alphanumeric_id() {
    assert!(parse("urn:oasis:names:tc:ebcore:partyid-type:iso6523:0208:0123.456-AB").is_ok());
  }

  #[test]
  fn rejects_structural_errors() {
    assert_eq!(
      parse("urn:oasis:names:tc:ebcore:partyid-type:unregistered:x"),
      Err(PartyIdError::InvalidPrefix)
    );
    assert_eq!(parse("urn:oasis"), Err(PartyIdError::InvalidPrefix));
    assert_eq!(
      parse("urn:oasis:names:tc:ebcore:partyid-type:iso6523:002:552100554"),
      Err(PartyIdError::InvalidIcd("002".into()))
    );
    assert_eq!(
      parse("urn:oasis:names:tc:ebcore:partyid-type:iso6523:00A2:1"),
      Err(PartyIdError::InvalidIcd("00A2".into()))
    );
    assert_eq!(
      parse("urn:oasis:names:tc:ebcore:partyid-type:iso6523:0002"),
      Err(PartyIdError::MissingOrganizationId)
    );
    assert_eq!(
      parse("urn:oasis:names:tc:ebcore:partyid-type:iso6523:0002:"),
      Err(PartyIdError::MissingOrganizationId)
    );
    assert_eq!(
      parse("urn:oasis:names:tc:ebcore:partyid-type:iso6523:0208:abc/def"),
      Err(PartyIdError::InvalidCharacter('/'))
    );
    assert_eq!(
      parse("urn:oasis:names:tc:ebcore:partyid-type:iso6523:0208:a:b"),
      Err(PartyIdError::InvalidCharacter(':'))
    );
    assert_eq!(
      parse(&format!("{ISO6523_PREFIX}0208:{}", "1".repeat(36))),
      Err(PartyIdError::OrganizationIdTooLong)
    );
  }

  #[test]
  fn validates_sirene_check_digits() {
    assert!(parse(&format!("{ISO6523_PREFIX}0002:552100555")).is_err());
    assert!(parse(&format!("{ISO6523_PREFIX}0002:5521005")).is_err());
    assert!(parse(&format!("{ISO6523_PREFIX}0009:55210055400013")).is_ok());
    assert!(parse(&format!("{ISO6523_PREFIX}0009:55210055400014")).is_err());
    // La Poste establishment, valid under the modulo 5 rule only.
    assert!(parse(&format!("{ISO6523_PREFIX}0009:35600000000048")).is_ok());
    assert!(parse(&format!("{ISO6523_PREFIX}0009:35600000049837")).is_ok());
  }

  #[test]
  fn validates_gln_and_duns() {
    assert!(parse(&format!("{ISO6523_PREFIX}0088:4035811991021")).is_ok());
    assert!(parse(&format!("{ISO6523_PREFIX}0088:4035811991022")).is_err());
    assert!(parse(&format!("{ISO6523_PREFIX}0060:123456789")).is_ok());
    assert!(parse(&format!("{ISO6523_PREFIX}0060:12345678")).is_err());
  }
}
