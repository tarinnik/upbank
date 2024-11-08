use crate::common::{Amount, Links, SelfLink};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccountList {
    pub data: Vec<AccountData>,
    pub links: Links,
}

impl AccountList {
    /// Gets the ID of the first transaction account if one exists.
    pub fn transaction_account_id(&self) -> Option<&String> {
        Some(
            &self
                .data
                .iter()
                .find(|acc| acc.attributes.account_type == AccountType::Transactional)?
                .id,
        )
    }
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
    pub balance: Amount,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Debug, PartialEq)]
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
