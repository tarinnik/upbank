use crate::common::{Amount, Links, SelfLink};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransactionList {
    pub data: Vec<TransactionData>,
}

#[derive(Deserialize)]
pub struct TransactionData {
    pub id: String,
}

#[derive(Deserialize)]
pub struct TransactionAttributes {
    pub status: TransactionStatus,
    #[serde(rename = "rawText")]
    pub raw_text: Option<String>,
    pub description: String,
    pub message: Option<String>,
    #[serde(rename = "isCategorizable")]
    pub is_categorisable: bool,
    #[serde(rename = "holdInfo")]
    pub hold_info: Option<TransactionHoldInfo>,
}

#[derive(Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "HELD")]
    Held,
    #[serde(rename = "SETTLED")]
    Settled,
}

#[derive(Deserialize)]
pub struct TransactionHoldInfo {
    pub amount: Amount,
    #[serde(rename = "foreignAmount")]
    pub foreign_amount: Option<Amount>,
}
