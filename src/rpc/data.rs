use ethers::providers::JsonRpcError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcLambdaRequest {
    pub payload: RpcPayload,
    pub relayer_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcPayload {
    pub id: i32,
    pub method: String,
    pub params: Value,
    pub jsonrpc: JsonRpcVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JsonRpcVersion {
    #[serde(rename = "2.0")]
    V2,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcResponse<R> {
    pub id: i32,
    pub jsonrpc: JsonRpcVersion,
    #[serde(default = "none", skip_serializing_if = "Option::is_none")]
    pub result: Option<R>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

fn none<R>() -> Option<R> {
    None
}
