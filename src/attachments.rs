use crate::common::{Links, SelfLink};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AttachmentList {
    pub data: Vec<AttachmentData>,
}

#[derive(Deserialize, Debug)]
pub struct Attachment {
    pub data: AttachmentData,
    pub links: Links,
}

#[derive(Deserialize, Debug)]
pub struct AttachmentData {
    pub id: String,
    pub attributes: AttachmentAttributes,
    pub relationships: AttachmentRelationships,
    pub links: Option<SelfLink>,
}

#[derive(Deserialize, Debug)]
pub struct AttachmentAttributes {
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "fileURL")]
    pub file_url: Option<String>,
    #[serde(rename = "fileURLExpiresAt")]
    pub file_url_expiration: Option<DateTime<FixedOffset>>,
    #[serde(rename = "fileExtension")]
    pub file_extension: Option<String>,
    #[serde(rename = "fileContentType")]
    pub file_content_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AttachmentRelationships {}

#[derive(Deserialize, Debug)]
pub struct RelationshipTransaction {
    pub data: TransactionData,
    pub links: Option<TransactionLinks>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionData {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionLinks {
    pub related: String,
}
