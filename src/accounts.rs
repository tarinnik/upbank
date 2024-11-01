use crate::common::{Links, SelfLink};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Accounts {
    pub data: Vec<AccountData>,
    pub links: Links,
}

#[derive(Deserialize)]
pub struct Account {
    pub data: AccountData,
}

#[derive(Deserialize)]
pub struct AccountData {
    #[serde(rename = "type")]
    pub account_type: String,
    pub id: String,
    pub attributes: AccountAttributes,
    pub relationships: AccountRelationships,
    pub links: Option<SelfLink>,
}

#[derive(Deserialize)]
pub struct AccountAttributes {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "accountType")]
    pub account_type: AccountType,
    #[serde(rename = "ownershipType")]
    pub ownership_type: AccountOwnership,
    pub balance: AccountBalance,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Deserialize)]
pub enum AccountType {
    #[serde(rename = "SAVER")]
    Saver,
    #[serde(rename = "TRANSACTIONAL")]
    Transactional,
    #[serde(rename = "HOME_LOAN")]
    HomeLoan,
}

#[derive(Deserialize)]
pub enum AccountOwnership {
    #[serde(rename = "INDIVIDUAL")]
    Individual,
    #[serde(rename = "JOINT")]
    Joint,
}

#[derive(Deserialize)]
pub struct AccountBalance {
    #[serde(rename = "currentCode")]
    pub currency_code: String,
    pub value: String,
    #[serde(rename = "valueInBaseUnits")]
    pub base_value: i64,
}

#[derive(Deserialize)]
pub struct AccountRelationships {
    pub transactions: AccountTransactions,
}

#[derive(Deserialize)]
pub struct AccountTransactions {
    pub links: Option<AccountTransactionsLink>,
}

#[derive(Deserialize)]
pub struct AccountTransactionsLink {
    pub related: String,
}
