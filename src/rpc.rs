use std::sync::Arc;

use async_trait::async_trait;
use aws_sdk_lambda::primitives::Blob;
use ethers::providers::JsonRpcClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

use self::error::ClientError;
use super::TxSitterInner;
use crate::rpc::data::{JsonRpcResponse, JsonRpcVersion, RpcLambdaRequest, RpcPayload};

pub mod data;
pub mod error;

#[derive(Clone, Debug)]
pub struct TxSitterRpcClient {
    relayer_id: String,
    inner: Arc<TxSitterInner>,
}

impl TxSitterRpcClient {
    pub(super) fn new(relayer_id: String, inner: Arc<TxSitterInner>) -> Self {
        Self { relayer_id, inner }
    }
}

#[async_trait]
impl JsonRpcClient for TxSitterRpcClient {
    type Error = ClientError;

    /// Sends a request with the provided JSON-RPC and parameters serialized as JSON
    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Self::Error>
    where
        T: std::fmt::Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let payload = RpcPayload {
            id: 1,
            method: method.into(),
            params: serde_json::to_value(params)?,
            jsonrpc: JsonRpcVersion::V2,
        };

        let lambda_request = RpcLambdaRequest {
            payload,
            relayer_id: self.relayer_id.clone(),
        };

        let payload = serde_json::to_vec(&lambda_request)?;

        let res = self
            .inner
            .client
            .invoke()
            .function_name(self.inner.config.rpc_lambda_name.clone())
            .payload(Blob::new(payload))
            .send()
            .await?;

        let payload = res.payload.ok_or(ClientError::MissingPayload)?;

        let result: JsonRpcResponse<R> = serde_json::from_slice(&payload.into_inner())?;

        match (result.result, result.error) {
            (Some(result), None) => Ok(result),
            (None, Some(error)) => Err(ClientError::Rpc(error)),
            _ => Err(ClientError::Other(
                "Invalid response from the RPC".to_string(),
            )),
        }
    }
}
