#[macro_use] extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
use diesel::{r2d2::{ConnectionManager, self}, MysqlConnection};
use std::sync::Arc;

mod app_config;
mod daos;
mod dtos;
mod models;
mod schema;
mod services;

use app_config::AppConfig;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let manager = ConnectionManager::<MysqlConnection>::new("mysql://root@localhost:3306/blog");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB connection pool");
    let app_config = Arc::new(AppConfig::new());

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

