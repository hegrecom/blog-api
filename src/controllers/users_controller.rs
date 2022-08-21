use actix_web::{post, HttpResponse, Responder, Result, web};

use crate::DbPool;
use crate::app_config::AppConfig;
use crate::dtos::{UserCreationDto, UserCreatedDto, UserSigningInDto, UserTokenCreatedDto};
use crate::services::{UserCreationService, UserSigningInService};

#[post("/sign_up")]
pub async fn sign_up(pool: web::Data<DbPool>, app_config: web::Data<AppConfig>, request_body: web::Json<UserCreationDto>) -> Result<impl Responder> {
    let user = web::block(move || {
        let conn = pool.get()?;
        let service = UserCreationService::new(&conn, request_body.into_inner(), &app_config.password_salt);
        service.run()
    })
    .await??;

    Ok(HttpResponse::Ok().json(UserCreatedDto::from(user)))
}


#[post("/sign_in")]
pub async fn sign_in(pool: web::Data<DbPool>, app_config: web::Data<AppConfig>, request_body: web::Json<UserSigningInDto>) -> Result<impl Responder> {
    let user_token = web::block(move || {
        let conn = pool.get()?;
        let service = UserSigningInService::new(&conn, request_body.into_inner(), &app_config.password_salt);
        service.run()
    })
    .await??;

    Ok(HttpResponse::Ok().json(UserTokenCreatedDto::from(user_token)))
}

