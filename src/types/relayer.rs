use serde::{Deserialize, Serialize};

use super::transaction::TransactionType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRelayerRequest {
    pub name: String,
    pub network: String,
    pub relayer_details: RelayerDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayerDetails {
    pub transaction_type: TransactionType,
    pub max_l1_gas_price: String,
    pub max_l2_gas_price: String,
}
