use std::str::FromStr;

use ed25519_dalek::{PublicKey, Signature, SignatureError, Verifier};
use hex::FromHexError;
use tiny_http::{Header, HeaderField};

pub(crate) fn verify_message(
    public_key: &str,
    headers: &[Header],
    body: &str,
) -> Result<(), VerificationError> {
    let header_signature = &HeaderField::from_str("X-Signature-Ed25519");
    let header_timestamp = &HeaderField::from_str("X-Signature-Timestamp");
    
    if header_signature.is_err() || header_timestamp.is_err() {
        return Err(VerificationError::HexParseFailed(FromHexError::OddLength))
    }
    
    
    let signature = headers.iter().find(|h| h.field.eq(&header_signature.clone().unwrap()));
    let timestamp = headers.iter().find(|h| h.field.eq(&header_timestamp.clone().unwrap()));
    
    if signature.is_none() || timestamp.is_none() {
        return Err(VerificationError::HexParseFailed(FromHexError::OddLength))
    }

    let public_key = &hex::decode(public_key)
        .map_err(VerificationError::HexParseFailed)
        .and_then(|bytes| {
            PublicKey::from_bytes(&bytes).map_err(VerificationError::InvalidSignature)
        })?;

    Ok(
        public_key.verify(
        format!("{}{}", timestamp.unwrap().value.to_string(), body).as_bytes(),
        &hex::decode(&signature.unwrap().value.to_string())
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
