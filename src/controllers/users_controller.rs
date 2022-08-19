use actix_web::{post, HttpResponse, Responder, Result, web};

use crate::DbPool;
use crate::app_config::AppConfig;
use crate::dtos::{UserCreationDto, UserCreatedDto};
use crate::services::UserCreationService;

#[post("/sign_up")]
pub async fn sign_up(pool: web::Data<DbPool>, app_config: web::Data<AppConfig>, request_body: web::Json<UserCreationDto>) -> Result<impl Responder> {
    let user = web::block(move || {
        let conn = pool.get()?;
        let service = UserCreationService::new(&conn, request_body.into_inner(), &app_config.password_salt);
        service.run()
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(UserCreatedDto::from(user)))
}

