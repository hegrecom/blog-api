use actix_web::{error, HttpResponse, http::{header::ContentType, StatusCode}};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum Exceptions {
    #[display(fmt = "{}", message)]
    BadRequest { message: &'static str },
    #[display(fmt = "{}", message)]
    Unauthorized { message: &'static str },
    #[display(fmt = "{}", message)]
    Forbidden { message: &'static str },
    #[display(fmt = "{}", message)]
    NotFound { message: &'static str },
    #[display(fmt = "{}", message)]
    Conflict { message: &'static str },
    #[display(fmt = "An internal error occurred. Please try again later")]
    InternalServerError,
}

impl error::ResponseError for Exceptions {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(serde_json::json!({
                "data": {},
                "meta": {
                    "message": self.to_string()
                }
            }))
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

impl From<diesel::result::Error> for Exceptions {
    fn from(_: diesel::result::Error) -> Self {
        Exceptions::InternalServerError
    }
}

impl From<bcrypt::BcryptError> for Exceptions {
    fn from(_: bcrypt::BcryptError) -> Self {
        Exceptions::InternalServerError
    }
}

impl From<r2d2::Error> for Exceptions {
    fn from(_: r2d2::Error) -> Self {
        Exceptions::InternalServerError
    }
}

