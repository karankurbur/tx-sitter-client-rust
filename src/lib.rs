use std::sync::Arc;

use aws_sdk_lambda as lambda;
use aws_sdk_lambda::primitives::Blob;
use eyre::Context;
use serde_json::Value;
use types::transaction::TxSitterTransaction;
use types::tx_sitter::{ReqByRelayerIdAndStatus, TransactionStatus};

use self::rpc::TxSitterRpcClient;
use crate::types::transaction::{TransactionResponse, TxSitterTransactionInput};
use crate::types::tx_sitter::{ReqTransactionById, TransactionRequest};

pub mod rpc;
pub mod types;

#[derive(Debug, Clone)]
pub struct TxSitterConfig {
    pub send_lambda_name: String,
    pub rpc_lambda_name: String,
    pub transactions_lambda_name: String,
}

#[derive(Clone, Debug)]
pub struct TxSitterClient {
    inner: Arc<TxSitterInner>,
}

#[derive(Debug)]
struct TxSitterInner {
    client: lambda::Client,
    config: TxSitterConfig,
}

impl TxSitterClient {
    pub async fn new(config: TxSitterConfig) -> Self {
        let aws_config = aws_config::load_from_env().await;
        let client = lambda::Client::new(&aws_config);

        let inner = Arc::new(TxSitterInner { client, config });

        Self { inner }
    }

    pub fn get_provider(&self, relayer_id: &str) -> TxSitterRpcClient {
        TxSitterRpcClient::new(relayer_id.into(), self.inner.clone())
    }

    pub async fn relay_transaction(
        &self,
        transaction: TxSitterTransactionInput,
    ) -> eyre::Result<String> {
        tracing::info!(?transaction, "Sending a tx-sitter transaction");

        let payload = serde_json::to_string(&transaction)?;

        let res = self
            .inner
            .client
            .invoke()
            .function_name(self.inner.config.send_lambda_name.clone())
            .payload(Blob::new(payload))
            .send()
            .await?;

        if let Some(payload_blob) = res.payload {
            let val = String::from_utf8(payload_blob.into_inner())?;
            let response: TransactionResponse =
                serde_json::from_str(&val).context("Invalid transaction response")?;

            Ok(response.result)
        } else {
            Err(eyre::eyre!("No payload found"))
        }
    }

    pub async fn get_transaction_by_relayer_and_status(
        &self,
        relayer_id: &str,
        status: TransactionStatus,
    ) -> eyre::Result<Vec<TxSitterTransactionInput>> {
        let get_tx_by_id_request = TransactionRequest::RelayerId(ReqByRelayerIdAndStatus {
            relayer_id: relayer_id.into(),
            status,
        });

        let payload = serde_json::to_string(&get_tx_by_id_request)?;

        let res = self
            .inner
            .client
            .invoke()
            .function_name(self.inner.config.transactions_lambda_name.clone())
            .payload(Blob::new(payload))
            .send()
            .await?;

        if let Some(payload_blob) = res.payload {
            let val = String::from_utf8(payload_blob.into_inner())?;

            Ok(serde_json::from_str(&val).context("Invalid transaction payload")?)
        } else {
            Err(eyre::eyre!("No payload found"))
        }
    }

    pub async fn get_transaction_by_id(&self, tx_id: &str) -> eyre::Result<TxSitterTransaction> {
        let get_tx_by_id_request = TransactionRequest::TransactionId(ReqTransactionById {
            transaction_id: tx_id.into(),
        });

        let payload = serde_json::to_string(&get_tx_by_id_request)?;

        let res = self
            .inner
            .client
            .invoke()
            .function_name(self.inner.config.transactions_lambda_name.clone())
            .payload(Blob::new(payload))
            .send()
            .await?;

        if let Some(payload_blob) = res.payload {
            let val = String::from_utf8(payload_blob.into_inner())?;
            let json: Value = serde_json::from_str(&val)?;

            let transactions = json
                .get("transactions")
                .and_then(|arr| arr.get(0))
                .ok_or_else(|| eyre::eyre!("Transaction not found or invalid format"))?;

            Ok(serde_json::from_value(transactions.clone())
                .context("Invalid transaction payload")?)
        } else {
            Err(eyre::eyre!("No payload found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::sync::Arc;

    use ethers::middleware::providers::Provider;
    use ethers::prelude::{abigen, H160};
    use ethers::providers::Middleware;
    use ethers::types::H256;

    use crate::types::transaction::{TransactionPriority, TxSitterTransactionInput};
    use crate::{TxSitterClient, TxSitterConfig};

    const ARN_SEND_LAMBDA: &str = "arn:aws:lambda:us-east-1:487223788601:function:TxSitter-staging-SendLambda3E928DB8-WenqWs8QhyxS";
    const ARN_RPC_LAMBDA: &str = "arn:aws:lambda:us-east-1:487223788601:function:TxSitter-staging-RpcLambda7CA8B6A4-Vs2aA9M1uONc";
    const ARN_TRANSACTION_LAMBDA: &str = "arn:aws:lambda:us-east-1:487223788601:function:TxSitter-staging-TransactionsLambda59148499-0IhhSKx9rR44";

    #[tokio::test]
    async fn test_relayer_transaction() -> eyre::Result<()> {
        let relayer_id = "e2c0c380-957a-4705-b4c0-d420fcc49de5";

        let tx_sitter_client = TxSitterClient::new(TxSitterConfig {
            send_lambda_name: ARN_SEND_LAMBDA.into(),
            rpc_lambda_name: ARN_RPC_LAMBDA.into(),
            transactions_lambda_name: ARN_TRANSACTION_LAMBDA.into(),
        })
        .await;

        let payload: TxSitterTransactionInput = TxSitterTransactionInput {
            to: "0x86C5608362B3fBBeB721140472229392f754eF87".to_string(),
            value: "8".to_string(),
            gas_limit: "200000".to_string(),
            relayer_id: relayer_id.into(),
            data: String::new(),
            transaction_id: None,
            priority: TransactionPriority::Fastest.into(),
            transaction_type: None,
        };

        let tx_id = tx_sitter_client.relay_transaction(payload).await?;

        println!("tx_id: {}", tx_id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_transaction() -> eyre::Result<()> {
        let tx_sitter_client = TxSitterClient::new(TxSitterConfig {
            send_lambda_name: ARN_SEND_LAMBDA.into(),
            rpc_lambda_name: ARN_RPC_LAMBDA.into(),
            transactions_lambda_name: ARN_TRANSACTION_LAMBDA.into(),
        })
        .await;

        let tx = tx_sitter_client
            .get_transaction_by_id(
                "0x6fdf5d47d291e9f494f3b8e0ba4901c8065eb549b14f070c9e9d0d5d265e18b1",
            )
            .await?;

        println!("tx_hash = {}", tx.tx_hash);

        Ok(())
    }

    #[tokio::test]
    async fn test_relayer_rpc() -> eyre::Result<()> {
        let relayer_id = "e2c0c380-957a-4705-b4c0-d420fcc49de5";

        let tx_sitter_client = TxSitterClient::new(TxSitterConfig {
            send_lambda_name: ARN_SEND_LAMBDA.into(),
            rpc_lambda_name: ARN_RPC_LAMBDA.into(),
            transactions_lambda_name: ARN_TRANSACTION_LAMBDA.into(),
        })
        .await;

        let rpc = tx_sitter_client.get_provider(relayer_id);
        let provider = Provider::new(rpc);

        let tx_hash =
            hex_literal::hex!("4a44d3d0e30d681741b8ed63c3f41560491123ae2ccd599078abd846f8575f2b");
        let tx_hash = H256(tx_hash);
        let tx = provider.get_transaction(tx_hash).await?;

        println!("tx: {tx:#?}",);

        abigen!(
            IERC20,
            r#"[
                function totalSupply() external view returns (uint256)
            ]"#;
        );

        let provider = Arc::new(provider);
        let address = H160::from_str("0xf132e6112e358a36fccf4660d082c9dae3da7411")?;
        let state_bridge = IERC20::new(address, provider.clone());
        let response = state_bridge.total_supply().call().await?;

        println!("Contract call response: {:#?}", response);

        Ok(())
    }
}
