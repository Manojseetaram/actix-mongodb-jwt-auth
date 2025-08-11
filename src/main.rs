use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{config::{get_env, load_config}, db::get_user_connection, handles::{login, register}};

mod config;
mod auth;
mod handles;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    load_config();
    env_logger::init();
    let mongo_uri = get_env("MONGO_URI");
    let mongodb = get_env("MONGO_DB");
    let jwt_secret = get_env("JWT_SECRET");
    let bind_address = get_env("BIND");



    let users = get_user_connection(&mongo_uri, &mongodb).await;
    let secret_data = web::Data::new(jwt_secret.clone());

     HttpServer::new(move || {
        App::new()
          .wrap(Logger::default())
          .app_data(web::Data::new(users.clone()))
          .app_data(secret_data.clone())
          .route("/register",web::post().to(register))
          .route("/login", web::post().to(login))
     })
     .bind(bind_address).expect("Server not ruuning")
     .run()
     .await

    
}
