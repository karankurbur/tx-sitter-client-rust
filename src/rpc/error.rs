use aws_sdk_lambda::operation::invoke::InvokeError;
use ethers::providers::{JsonRpcError, ProviderError, RpcError};

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Aws(
        #[from]
        ::aws_smithy_http::result::SdkError<
            InvokeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    ),
    #[error("No payload returned from the RPC")]
    MissingPayload,

    #[error("RPC returned with an error: {0}")]
    Rpc(JsonRpcError),

    #[error("{0}")]
    Other(String),
}

impl RpcError for ClientError {
    fn as_error_response(&self) -> Option<&ethers::providers::JsonRpcError> {
        if let Self::Rpc(value) = self {
            Some(value)
        } else {
            None
        }
    }

    fn as_serde_error(&self) -> Option<&serde_json::Error> {
        if let Self::SerdeJson(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

impl From<ClientError> for ProviderError {
    fn from(value: ClientError) -> Self {
        match value {
            ClientError::SerdeJson(serde_json) => ProviderError::SerdeJson(serde_json),
            other => {
                let s = other.to_string();
                ProviderError::CustomError(s)
            }
        }
    }
}
