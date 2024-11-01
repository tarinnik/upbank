use serde::Deserialize;

#[derive(Deserialize)]
pub struct Links {
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Deserialize)]
pub struct SelfLink {
    #[serde(rename = "self")]
    pub self_link: String,
}
