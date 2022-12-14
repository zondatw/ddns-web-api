use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorSerializer {
    code: u16,
    error: String,
    message: String,
}

#[derive(Debug, Display, Error)]
pub enum ErrorResponder {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl ErrorResponder {
    pub fn name(&self) -> String {
        match self {
            Self::InternalError => "Internel Error".to_string(),
            Self::BadClientData => "Bad Client Data".to_string(),
            Self::Timeout => "Timeout".to_string(),
        }
    }
}

impl error::ResponseError for ErrorResponder {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorSerializer {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadClientData => StatusCode::BAD_REQUEST,
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
