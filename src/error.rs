use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("up error")]
    Up(#[from] UpError),
    #[error("request error")]
    Request(#[from] reqwest::Error),
}

#[derive(Deserialize, Debug, Error)]
#[error("{} up error{}", .errors.len(), if .errors.len() == 0 {""} else {"s"})]
pub struct UpErr {
    pub errors: Vec<UpError>,
}

#[derive(Deserialize, Debug, Error)]
#[error("up error {} - {}", .status, .title)]
pub struct UpError {
    pub status: String,
    pub title: String,
    pub detail: String,
    pub source: Option<UpErrorSource>,
}

#[derive(Deserialize, Debug, Error)]
#[error("up error source {:?} - {:?}", .parameter, .pointer)]
pub struct UpErrorSource {
    pub parameter: Option<String>,
    pub pointer: Option<String>,
}
