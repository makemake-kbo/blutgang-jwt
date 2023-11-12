use serde::{
    Deserialize,
    Serialize,
};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: Value,
    pub jsonrpc: Value,
    pub method: Value,
    pub params: Value,
    pub exp: usize,
}
