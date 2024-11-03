use crate::{
    common::{Links, SelfLink},
    currencies::currency_map,
};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccountList {
    pub data: Vec<AccountData>,
    pub links: Links,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub data: AccountData,
}

#[derive(Deserialize, Debug)]
pub struct AccountData {
    pub id: String,
    pub attributes: AccountAttributes,
    pub relationships: AccountRelationships,
    pub links: Option<SelfLink>,
}

#[derive(Deserialize, Debug)]
pub struct AccountAttributes {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "accountType")]
    pub account_type: AccountType,
    #[serde(rename = "ownershipType")]
    pub ownership_type: AccountOwnership,
    pub balance: AccountBalance,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Debug)]
pub enum AccountType {
    #[serde(rename = "SAVER")]
    Saver,
    #[serde(rename = "TRANSACTIONAL")]
    Transactional,
    #[serde(rename = "HOME_LOAN")]
    HomeLoan,
}

#[derive(Deserialize, Debug)]
pub enum AccountOwnership {
    #[serde(rename = "INDIVIDUAL")]
    Individual,
    #[serde(rename = "JOINT")]
    Joint,
}

#[derive(Deserialize, Debug)]
pub struct AccountRelationships {
    pub transactions: AccountTransactions,
}

#[derive(Deserialize, Debug)]
pub struct AccountTransactions {
    pub links: Option<AccountTransactionsLink>,
}

#[derive(Deserialize, Debug)]
pub struct AccountTransactionsLink {
    pub related: String,
}
