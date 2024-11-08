use crate::common::{Amount, Links, SelfLink};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TransactionList {
    pub data: Vec<TransactionData>,
    pub links: Links,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub data: TransactionData,
}

#[derive(Deserialize, Debug)]
pub struct TransactionData {
    pub id: String,
    pub attributes: TransactionAttributes,
    pub relationships: TransactionRelationships,
    pub links: SelfLink,
}

#[derive(Deserialize, Debug)]
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
    #[serde(rename = "roundUp")]
    pub round_up: Option<TransactionRoundUp>,
    pub cashback: Option<TransactionCashback>,
    pub amount: Amount,
    #[serde(rename = "foreignAmount")]
    pub foreign_amount: Option<Amount>,
    #[serde(rename = "cardPurchaseMethod")]
    pub card_purchase_method: Option<TransactionMethodData>,
    #[serde(rename = "settledAt")]
    pub settled_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "transactionType")]
    pub transaction_type: Option<String>,
    pub note: Option<TransactionNote>,
    #[serde(rename = "performingCustomer")]
    pub performing_customer: Option<TransactionPerformingCustomer>,
    #[serde(rename = "deepLinkUrl")]
    pub deep_link_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionRelationships {
    pub account: TransactionRelationship,
    #[serde(rename = "transferAccount")]
    pub transfer_account: TransactionRelationship,
    pub category: TransactionRelationship,
    #[serde(rename = "parentCategory")]
    pub parent_categotry: TransactionRelationship,
    pub tags: TransactionTagRelationship,
    pub attachment: TransactionRelationship,
}

#[derive(Deserialize, Debug)]
pub enum TransactionStatus {
    #[serde(rename = "HELD")]
    Held,
    #[serde(rename = "SETTLED")]
    Settled,
}

#[derive(Deserialize, Debug)]
pub struct TransactionHoldInfo {
    pub amount: Amount,
    #[serde(rename = "foreignAmount")]
    pub foreign_amount: Option<Amount>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionRoundUp {
    pub amount: Amount,
    #[serde(rename = "boostPortion")]
    pub boost_portion: Option<Amount>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionCashback {
    pub description: String,
    pub amount: Amount,
}

#[derive(Deserialize, Debug)]
pub struct TransactionMethodData {
    pub method: TransactionMethod,
    #[serde(rename = "cardNumberSuffix")]
    pub card_number_suffix: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum TransactionMethod {
    #[serde(rename = "BAR_CODE")]
    BarCode,
    #[serde(rename = "OCR")]
    Ocr,
    #[serde(rename = "CARD_PIN")]
    CardPin,
    #[serde(rename = "CARD_DETAILS")]
    CardDetails,
    #[serde(rename = "CARD_ON_FILE")]
    CardOnFile,
    #[serde(rename = "ECOMMERCE")]
    Ecommerce,
    #[serde(rename = "MAGNETIC_STRIPE")]
    MagneticStripe,
    #[serde(rename = "CONTACTLESS")]
    Contactless,
}

#[derive(Deserialize, Debug)]
pub struct TransactionNote {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionPerformingCustomer {
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionRelationship {
    pub data: Option<TransactionRelationshipData>,
    pub links: Option<TransactionRelationshipLinks>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionTagRelationship {
    pub data: Vec<TransactionRelationshipData>,
    pub links: Option<TransactionRelationshipLinks>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionRelationshipData {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionRelationshipLinks {
    pub related: Option<String>,
    #[serde(rename = "self")]
    pub self_link: Option<String>,
}
