use axum::{http::StatusCode, response::IntoResponse, Json};
use community::services::responses::ServiceError;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct AxumResponse<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for AxumResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(json!(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct AxumError(pub ServiceError);
impl IntoResponse for AxumError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self.0 {
            ServiceError::BaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "base_error"),
            ServiceError::AuthenticationError(_) => {
                (StatusCode::UNAUTHORIZED, "authentication_error")
            }
            ServiceError::InvalidPassword(reason) => {
                // TODO: pass error code to resp
                (StatusCode::BAD_REQUEST, "invalid_password")
            }
            ServiceError::HashLibError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error")
            }
            ServiceError::JWTError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error")
            }
        };

        let body = Json(json!({"message":msg}));
        (status, body).into_response()
    }
}

impl From<ServiceError> for AxumError {
    fn from(value: ServiceError) -> Self {
        Self(value)
    }
}
