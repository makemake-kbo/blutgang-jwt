use serde_json::Value;
use serde::{
    Deserialize,
    Serialize
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: Value,
    pub jsonrpc: Value,
    pub method: Value,
    pub params: Value,
    pub exp: usize,
}
