//! AFiR retrieval receipt, ML-DSA-65 profile: a node's signed record that it sent
//! one request to one external data source and received one response body.
//!
//! Byte layout: eight fields concatenated in listing order at fixed widths,
//! integers big-endian, 140 bytes. ML-DSA-65 signs them under [`CONTEXT`].
//!
//! Blinded commitments: `request_hash` and `response_hash` are keyed BLAKE3
//! under a random per-receipt salt that the node keeps with the stored body.

use fips204::{
    ml_dsa_65,
    traits::{SerDes, Verifier},
};

pub const VERSION: u8 = 0x01;
pub const PROFILE_MLDSA65: u8 = 0x01;
pub const PAYLOAD_LEN: usize = 140;
pub const SIGNATURE_LEN: usize = 3309;
pub const SIGNED_LEN: usize = PAYLOAD_LEN + SIGNATURE_LEN;
pub const PUBLIC_KEY_LEN: usize = 1952;
pub const CONTEXT: &[u8] = b"ARKHE_AFIR_RETRIEVAL_V1";
pub const DELIVERY_CONTEXT: &[u8] = b"ARKHE_AFIR_DELIVERY_V1";
pub const PAYMENT_DOMAIN: &[u8] = b"ARKHE_X402_PAYMENT_V1";
pub const SOURCE_DOMAIN: &[u8] = b"ARKHE_RETRIEVAL_SOURCE_V1";
pub const REQUEST_DOMAIN: &[u8] = b"ARKHE_RETRIEVAL_REQUEST_V1";
pub const RESPONSE_DOMAIN: &[u8] = b"ARKHE_RETRIEVAL_RESPONSE_V1";
pub const SALT_LEN: usize = 32;
pub const CREDENTIAL_KEYS: [&str; 4] = ["token", "apikey", "api_key", "access_token"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("signed receipt must be {SIGNED_LEN} bytes")]
    Length,
    #[error("unsupported receipt version")]
    Version,
    #[error("unsupported receipt profile")]
    Profile,
    #[error("http_status is outside 100..=599")]
    Status,
    #[error("session_nonce must not be all zero")]
    Nonce,
    #[error("invalid ML-DSA-65 public key")]
    PublicKey,
    #[error("invalid ML-DSA-65 signature")]
    Signature,
    #[error("request carries a credential in its query")]
    Credential,
    #[error("request is not in canonical form")]
    RequestForm,
    #[error("blinding salt is missing or all zero")]
    Salt,
    #[error("receipt does not match host expectation: {0}")]
    Mismatch(&'static str),
}

impl Error {
    pub fn code(&self) -> &'static str {
        match self {
            Error::Length => "LENGTH",
            Error::Version => "VERSION",
            Error::Profile => "PROFILE",
            Error::Status => "STATUS",
            Error::Nonce => "NONCE",
            Error::PublicKey => "PUBLIC_KEY",
            Error::Signature => "SIGNATURE",
            Error::Credential => "CREDENTIAL",
            Error::RequestForm => "REQUEST_FORM",
            Error::Salt => "SALT",
            Error::Mismatch(field) => field,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Receipt {
    pub policy_id: [u8; 16],
    pub source_fp: [u8; 32],
    pub request_hash: [u8; 32],
    pub response_hash: [u8; 32],
    pub http_status: u16,
    pub session_nonce: [u8; 16],
    pub timestamp: u64,
}

impl Receipt {
    pub fn payload(&self) -> [u8; PAYLOAD_LEN] {
        let mut out = [0u8; PAYLOAD_LEN];
        let mut at = 0;
        for part in [
            &[VERSION, PROFILE_MLDSA65][..],
            &self.policy_id,
            &self.source_fp,
            &self.request_hash,
            &self.response_hash,
            &self.http_status.to_be_bytes(),
            &self.session_nonce,
            &self.timestamp.to_be_bytes(),
        ] {
            out[at..at + part.len()].copy_from_slice(part);
            at += part.len();
        }
        out
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "version": VERSION,
            "profile": PROFILE_MLDSA65,
            "policy_id": hex::encode(self.policy_id),
            "source_fp": hex::encode(self.source_fp),
            "request_hash": hex::encode(self.request_hash),
            "response_hash": hex::encode(self.response_hash),
            "http_status": self.http_status,
            "session_nonce": hex::encode(self.session_nonce),
            "timestamp": self.timestamp,
        })
    }

    pub fn decode_payload(bytes: &[u8; PAYLOAD_LEN]) -> Result<Self, Error> {
        if bytes[0] != VERSION {
            return Err(Error::Version);
        }
        if bytes[1] != PROFILE_MLDSA65 {
            return Err(Error::Profile);
        }
        let mut at = 2;
        let mut take = |n: usize| {
            let part = &bytes[at..at + n];
            at += n;
            part
        };
        let receipt = Receipt {
            policy_id: take(16).try_into().unwrap(),
            source_fp: take(32).try_into().unwrap(),
            request_hash: take(32).try_into().unwrap(),
            response_hash: take(32).try_into().unwrap(),
            http_status: u16::from_be_bytes(take(2).try_into().unwrap()),
            session_nonce: take(16).try_into().unwrap(),
            timestamp: u64::from_be_bytes(take(8).try_into().unwrap()),
        };
        if !(100..=599).contains(&receipt.http_status) {
            return Err(Error::Status);
        }
        if receipt.session_nonce == [0; 16] {
            return Err(Error::Nonce);
        }
        Ok(receipt)
    }
}

fn put(out: &mut Vec<u8>, field: &[u8]) -> Result<(), Error> {
    let len = u32::try_from(field.len()).map_err(|_| Error::RequestForm)?;
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(field);
    Ok(())
}

pub fn source_fingerprint(base_url: &str) -> [u8; 32] {
    let mut out = SOURCE_DOMAIN.to_vec();
    put(&mut out, base_url.as_bytes()).expect("host-configured base URL longer than 4 GiB");
    *blake3::hash(&out).as_bytes()
}

#[derive(Debug, Clone, Copy)]
pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub query: &'a [(&'a str, &'a str)],
    pub body: &'a [u8],
}

fn check_salt(salt: &[u8; SALT_LEN]) -> Result<(), Error> {
    if *salt == [0; SALT_LEN] {
        return Err(Error::Salt);
    }
    Ok(())
}

pub fn request_hash(salt: &[u8; SALT_LEN], request: &Request<'_>) -> Result<[u8; 32], Error> {
    check_salt(salt)?;
    let method_ok =
        !request.method.is_empty() && request.method.bytes().all(|b| b.is_ascii_uppercase());
    let path_ok = request.path.starts_with('/')
        && !request.path.contains(['?', '#'])
        && !request.path.split('/').any(|s| s == "." || s == "..");
    if !method_ok || !path_ok {
        return Err(Error::RequestForm);
    }
    if request
        .query
        .iter()
        .any(|(k, _)| CREDENTIAL_KEYS.contains(&k.to_ascii_lowercase().as_str()))
    {
        return Err(Error::Credential);
    }
    let mut pairs = request.query.to_vec();
    pairs.sort_unstable_by(|a, b| {
        (a.0.as_bytes(), a.1.as_bytes()).cmp(&(b.0.as_bytes(), b.1.as_bytes()))
    });
    let mut out = REQUEST_DOMAIN.to_vec();
    put(&mut out, request.method.as_bytes())?;
    put(&mut out, request.path.as_bytes())?;
    let count = u32::try_from(pairs.len()).map_err(|_| Error::RequestForm)?;
    out.extend_from_slice(&count.to_be_bytes());
    for (k, v) in pairs {
        put(&mut out, k.as_bytes())?;
        put(&mut out, v.as_bytes())?;
    }
    put(&mut out, request.body)?;
    Ok(*blake3::keyed_hash(salt, &out).as_bytes())
}

pub fn response_hash(salt: &[u8; SALT_LEN], body: &[u8]) -> Result<[u8; 32], Error> {
    check_salt(salt)?;
    let mut out = RESPONSE_DOMAIN.to_vec();
    put(&mut out, body)?;
    Ok(*blake3::keyed_hash(salt, &out).as_bytes())
}

pub fn payment_nonce(payment_payload: &[u8]) -> [u8; 16] {
    let mut out = PAYMENT_DOMAIN.to_vec();
    put(&mut out, payment_payload).expect("payment payload longer than 4 GiB");
    blake3::hash(&out).as_bytes()[..16].try_into().unwrap()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Expectation<'a> {
    pub policy_id: Option<[u8; 16]>,
    pub source_fp: Option<[u8; 32]>,
    pub session_nonce: Option<[u8; 16]>,
    pub salt: Option<[u8; SALT_LEN]>,
    pub request: Option<Request<'a>>,
    pub response: Option<&'a [u8]>,
}

pub fn verify(
    signed: &[u8],
    public_key: &[u8],
    expect: &Expectation<'_>,
) -> Result<Receipt, Error> {
    verify_under(CONTEXT, signed, public_key, expect)
}

pub fn verify_delivery(
    signed: &[u8],
    public_key: &[u8],
    expect: &Expectation<'_>,
) -> Result<Receipt, Error> {
    verify_under(DELIVERY_CONTEXT, signed, public_key, expect)
}

fn verify_under(
    context: &[u8],
    signed: &[u8],
    public_key: &[u8],
    expect: &Expectation<'_>,
) -> Result<Receipt, Error> {
    if signed.len() != SIGNED_LEN {
        return Err(Error::Length);
    }
    let payload: &[u8; PAYLOAD_LEN] = signed[..PAYLOAD_LEN].try_into().unwrap();
    let receipt = Receipt::decode_payload(payload)?;
    let key: [u8; PUBLIC_KEY_LEN] = public_key.try_into().map_err(|_| Error::PublicKey)?;
    let key = ml_dsa_65::PublicKey::try_from_bytes(key).map_err(|_| Error::PublicKey)?;
    let sig: &[u8; SIGNATURE_LEN] = signed[PAYLOAD_LEN..].try_into().unwrap();
    if !key.verify(payload, sig, context) {
        return Err(Error::Signature);
    }
    if expect.policy_id.is_some_and(|v| v != receipt.policy_id) {
        return Err(Error::Mismatch("POLICY_ID"));
    }
    if expect.source_fp.is_some_and(|v| v != receipt.source_fp) {
        return Err(Error::Mismatch("SOURCE_FP"));
    }
    if expect
        .session_nonce
        .is_some_and(|v| v != receipt.session_nonce)
    {
        return Err(Error::Mismatch("SESSION_NONCE"));
    }
    if expect.request.is_none() && expect.response.is_none() {
        return Ok(receipt);
    }
    let salt = expect.salt.ok_or(Error::Salt)?;
    if let Some(request) = &expect.request {
        if request_hash(&salt, request)? != receipt.request_hash {
            return Err(Error::Mismatch("REQUEST"));
        }
    }
    if let Some(body) = expect.response {
        if response_hash(&salt, body)? != receipt.response_hash {
            return Err(Error::Mismatch("RESPONSE_CONTENT"));
        }
    }
    Ok(receipt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_round_trip() {
        let receipt = Receipt {
            policy_id: [1u8; 16],
            source_fp: [2u8; 32],
            request_hash: [3u8; 32],
            response_hash: [4u8; 32],
            http_status: 200,
            session_nonce: [5u8; 16],
            timestamp: 1793577600,
        };
        let payload = receipt.payload();
        assert_eq!(payload.len(), PAYLOAD_LEN);
        let decoded = Receipt::decode_payload(&payload).expect("decode failed");
        assert_eq!(receipt, decoded);
    }

    #[test]
    fn test_refuse_credential_keys() {
        let salt = [7u8; 32];
        let req = Request {
            method: "GET",
            path: "/api/v1/data",
            query: &[("token", "secret123")],
            body: &[],
        };
        assert_eq!(request_hash(&salt, &req).unwrap_err(), Error::Credential);
    }

    #[test]
    fn test_refuse_zero_salt() {
        let salt = [0u8; 32];
        let req = Request {
            method: "GET",
            path: "/api/v1/data",
            query: &[],
            body: &[],
        };
        assert_eq!(request_hash(&salt, &req).unwrap_err(), Error::Salt);
        assert_eq!(response_hash(&salt, b"data").unwrap_err(), Error::Salt);
    }
}
