use std::str::FromStr;

use sodiumoxide::crypto::sign;
use sodiumoxide::hex;
use tiny_http::{HeaderField, Request};

pub fn verify_message(mut request: Request, public_key: &str) -> bool {
    let signature_header = &mut request.headers().iter().filter(|h| h.field.eq(&HeaderField::from_str("X-Signature-Ed25519").unwrap())).next().unwrap();
    let timestamp_header = &mut request.headers().iter().filter(|h| h.field.eq(&HeaderField::from_str("X-Signature-Timestamp").unwrap())).next().unwrap();

    let signature = signature_header.value.as_str();
    let timestamp = timestamp_header.value.as_str();

    let mut input = String::new();
    request.as_reader().read_to_string(&mut input).unwrap();

    let message = format!("{}{}", timestamp, input);

    sign::verify_detached(
        &sign::Signature::from_str(message.clone().as_str()).unwrap(),
        signature.as_bytes(),
        &sign::PublicKey::from_slice(&hex::decode(public_key).unwrap()).unwrap()
    )
}
