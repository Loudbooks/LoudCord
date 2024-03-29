use std::str::FromStr;

use ed25519_dalek::{PublicKey, Signature, SignatureError, Verifier};
use hex::FromHexError;
use tiny_http::{Header, HeaderField};

pub(crate) fn verify_message(
    public_key: &str,
    headers: &[Header],
    body: &str,
) -> Result<(), VerificationError> {
    let signature = headers.iter().find(|h| h.field.eq(&HeaderField::from_str("X-Signature-Ed25519").unwrap())).unwrap().value.to_string();
    let timestamp = headers.iter().find(|h| h.field.eq(&HeaderField::from_str("X-Signature-Timestamp").unwrap())).unwrap().value.to_string();

    let public_key = &hex::decode(public_key)
        .map_err(VerificationError::HexParseFailed)
        .and_then(|bytes| {
            PublicKey::from_bytes(&bytes).map_err(VerificationError::InvalidSignature)
        })?;

    Ok(
        public_key.verify(
        format!("{}{}", timestamp, body).as_bytes(),
        &hex::decode(&signature)
            .map_err(VerificationError::HexParseFailed)
            .and_then(|bytes| {
                Signature::from_bytes(&bytes).map_err(VerificationError::InvalidSignature)
            })?,
    )?)
}


#[derive(Debug, thiserror::Error)]
pub(crate) enum VerificationError {
    #[error("Failed to parse from hex.")]
    HexParseFailed(#[from] FromHexError),

    #[error("Invalid public key provided.")]
    InvalidPublicKey(#[from] SignatureError),

    #[error("Invalid signature provided.")]
    InvalidSignature(ed25519_dalek::ed25519::Error),
}