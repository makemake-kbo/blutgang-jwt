use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: Value,
    jsonrpc: Value,
    method: Value,
    params: Value,
    exp: usize,
}

fn main() {
    let a = Claims {
        id: Value::Number(1.into()),
        jsonrpc: Value::String("2.0".to_string()),
        method: Value::String("blutgang_ttl".to_string()),
        params: Value::Array(vec![]),
        exp: 10000000000,
    };

    let token = encode(
        &Header::default(),
        &a,
        &EncodingKey::from_secret(b"hashdsuahdbaoidyasv2218eyahbckncz210u30uhdbakuhasb"),
    )
    .unwrap();
    println!("Encoded: {}", token);
    // also decode
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(b"hashdsuahdbaoidyasv2218eyahbckncz210u30uhdbakuhasb"),
        &Validation::default(),
    )
    .unwrap();
    println!("Decoded: {:?}", decoded.claims);
}
