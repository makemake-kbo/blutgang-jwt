use crate::Claims;
use std::error::Error;

use jsonwebtoken::{
	encode,
	EncodingKey,
	Header,
};


pub fn sign(
	arg: Claims,
	token: &str,
) -> Result<String, SignError> {
	Ok(encode(&Header::default(), &arg,  &EncodingKey::from_secret(token.as_bytes()))?)
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

