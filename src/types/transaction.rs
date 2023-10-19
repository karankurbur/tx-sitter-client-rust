use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TxSitterTransactionInput {
    pub to: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub data: String,
    pub value: String,
    pub gas_limit: String,
    pub relayer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TransactionPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct TransactionResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct TxSitterTransaction {
    pub id: String,
    pub to: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub data: String,
    pub value: String,
    pub gas_limit: String,
    pub relayer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TransactionPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
    pub tx_hash: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TransactionPriority {
    Slowest,
    Slow,
    Regular,
    Fast,
    Fastest,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TransactionType {
    All,
    Swap,
    Transfer,
    Drop,
    Grant,
    Funding,
    WalletDeployment,
    RootPropagation,
    Noop,
    Bundle
}
