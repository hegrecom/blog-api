#[macro_use] extern crate diesel;

use actix_web::{web, App, HttpServer, middleware};
use diesel::{r2d2::{ConnectionManager, self}, MysqlConnection};

mod app_config;
mod controllers;
mod daos;
mod dtos;
mod exceptions;
mod models;
mod middlewares;
mod schema;
mod services;

use app_config::AppConfig;
use controllers::users_controller;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let manager = ConnectionManager::<MysqlConnection>::new("mysql://root@localhost:3306/blog");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB connection pool");
    let app_config = web::Data::new(AppConfig::new());
    let json_configuration = web::JsonConfig::default().error_handler(exceptions::json_error_handler);

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(app_config.clone())
            .app_data(json_configuration.clone())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/users")
                    .service(users_controller::sign_up)
                    .service(users_controller::sign_in),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

