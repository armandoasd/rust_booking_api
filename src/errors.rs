use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use ntex::web::{HttpRequest, HttpResponse, WebResponseError};
use std::convert::From;
use orion::errors::UnknownCryptoError;
use std::string::FromUtf8Error;
use serde::Serialize;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized: {}", _0)]
    Unauthorized(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl WebResponseError for ServiceError {
    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError()
                .json(&ErrorResponse{error:"Internal Server Error, Please try later".to_string()}),
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(&ErrorResponse{error:message.to_string()})
            }
            ServiceError::Unauthorized(ref message) => {
                HttpResponse::Unauthorized().json(&ErrorResponse{error:format!("Unauthorized: {}", message)})
            }
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message =
                        info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}


impl From<UnknownCryptoError> for ServiceError {
    fn from(error: UnknownCryptoError) -> ServiceError {
        return ServiceError::BadRequest(format!("malformed request :{}",error));
    }
}
impl From<FromUtf8Error> for ServiceError {
    fn from(error: FromUtf8Error) -> ServiceError {
        // return ServiceError::InternalServerError;
        return ServiceError::BadRequest(format!("malformed request :{}",error));
    }
}