use actix_web::{
    dev::HttpResponseBuilder,
    HttpResponse,
    ResponseError,
};

#[derive(Debug)]
pub enum ApplicationError {
    NotFound,
    BadRequest,
    Internal,
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<h1>{:?}</h1>", self)
    }
}

use actix_web::http::{
    header,
    StatusCode,
};
impl ResponseError for ApplicationError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApplicationError::BadRequest => StatusCode::BAD_REQUEST,
            ApplicationError::NotFound => StatusCode::NOT_FOUND,
            ApplicationError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
}
