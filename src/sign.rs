use crate::Claims;
use std::error::Error;

use jsonwebtoken::{
    encode,
    EncodingKey,
    Header,
};

pub fn sign(arg: Claims, token: &str) -> Result<String, SignError> {
    Ok(encode(
        &Header::default(),
        &arg,
        &EncodingKey::from_secret(token.as_bytes()),
    )?)
}

// Errors

#[derive(Debug)]
#[allow(dead_code)]
pub enum SignError {
    SignFailed(String),
}

impl std::fmt::Display for SignError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SignError::SignFailed(err) => write!(f, "Failed to sign JWT claim: {}", err),
        }
    }
}

impl Error for SignError {}

impl From<jsonwebtoken::errors::Error> for SignError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        SignError::SignFailed(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::decode;
    use jsonwebtoken::Algorithm;
    use jsonwebtoken::DecodingKey;
    use jsonwebtoken::Validation;
    use serde_json::Value;

    fn dummy_claims() -> Claims {
        Claims {
            id: Value::Number(1.into()),
            jsonrpc: Value::String("2.0".to_string()),
            method: Value::String("blutgang_ttl".to_string()),
            params: Value::Array(vec![]),
            exp: 10000000000,
        }
    }

    #[test]
    fn test_sign_and_verify() {
        // Arrange
        let token = "secret_token";
        let claims = dummy_claims();

        // Act
        let signed_token = sign(claims.clone(), token).unwrap();

        // Decode the token to verify
        let decoded_token = decode::<Claims>(
            &signed_token,
            &DecodingKey::from_secret(token.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .unwrap();

        // Assert
        assert_eq!(decoded_token.claims, claims);
    }
}
