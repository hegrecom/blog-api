use actix_web::{error, HttpResponse, http::{header::ContentType, StatusCode}};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum Exceptions {
    #[display(fmt = "{}", message)]
    BadRequest { message: String },
    #[display(fmt = "{}", message)]
    Unauthorized { message: String },
    #[display(fmt = "{}", message)]
    Forbidden { message: String },
    #[display(fmt = "{}", message)]
    NotFound { message: String },
    #[display(fmt = "{}", message)]
    Conflict { message: String },
    #[display(fmt = "An internal error occurred. Please try again later")]
    InternalServerError,
}

impl error::ResponseError for Exceptions {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Exceptions::BadRequest { message: _ } => StatusCode::BAD_REQUEST,
            Exceptions::Unauthorized { message: _ } => StatusCode::UNAUTHORIZED,
            Exceptions::Forbidden { message: _ } => StatusCode::FORBIDDEN,
            Exceptions::NotFound { message: _ } => StatusCode::NOT_FOUND,
            Exceptions::Conflict { message: _ } => StatusCode::CONFLICT,
            Exceptions::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

