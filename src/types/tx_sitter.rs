use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "by")]
pub enum TransactionRequest {
    TransactionId(ReqTransactionById),
    RelayerId(ReqByRelayerIdAndStatus),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReqTransactionById {
    pub transaction_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReqByRelayerIdAndStatus {
    pub relayer_id: String,
    pub status: TransactionStatus,
}

#[derive(PartialEq, Debug, Clone, Copy, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TransactionStatus {
    /// A transaction is waiting to be broadcast
    #[default]
    Queued,
    /// A transaction has been broadcast and is pending confirmation
    Pending,
    /// A transaction has been included in a block
    Mined,
    /// A transaction has been included in a block which is older
    /// than an arbitrary threshold (e.g. 10 blocks) depending on the network.
    Finalized,
    /// The transaction failed to be broadcast
    /// will not be retried
    Dropped,
}
