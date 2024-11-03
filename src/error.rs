use serde::Deserialize;
use thiserror::Error;

#[cfg(feature = "axum")]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("up error")]
    Up(#[from] UpError),
    #[error("request error")]
    Request(#[from] reqwest::Error),
}

#[cfg(feature = "axum")]
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Up(up) => up.into_response(),
            Error::Request(req) => {
                let code = req.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                let body = format!("{}", req);
                (code, body).into_response()
            }
        }
    }
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

#[cfg(feature = "axum")]
impl IntoResponse for UpError {
    fn into_response(self) -> Response {
        let code = StatusCode::from_u16(self.status.parse().unwrap_or(500))
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = format!("{} - {}", self.title, self.detail);
        (code, body).into_response()
    }
}

#[derive(Deserialize, Debug, Error)]
#[error("up error source {:?} - {:?}", .parameter, .pointer)]
pub struct UpErrorSource {
    pub parameter: Option<String>,
    pub pointer: Option<String>,
}
