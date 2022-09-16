extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use dotenvy::dotenv;
use std::env;

use actix_web::{
    delete, get, patch, post, put,
    web::{self, Data, Json, Path},
    App, HttpResponse, HttpServer, Responder, HttpRequest
};
use actix::SyncArbiter;      //arbiters manage actors
use db_utils::{get_pool};

use actors::db::{DbActor, Create, Update, Delete, GetAll};
use models::{AppState, UserPayload, NamePayload};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

mod db_utils;
mod schema;
mod models;
mod actors;

/* ENDPOINTS */

#[post("/user/new")]
async fn create_user(req: Json<UserPayload>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();                                             // get address of actor
    let body = req.into_inner();                                                    // unwrap json body
    match db.send(Create { name: body.name, age: body.age }).await {                // send message to actor, then
        Ok(Ok(user)) => HttpResponse::Ok().json(user),                              // if message was received AND db query was successful, success
        _ => HttpResponse::InternalServerError().json("Something went wrong.")      // otherwise...
    }
}

#[patch("/{userid}")]
async fn patch_user(userid: Path<Uuid>, req: Json<NamePayload>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let body = req.into_inner();
    match db.send(Update { uuid: *userid, name: body.name }).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong.")
    }
}

#[delete("/{userid}")]
async fn delete_user(userid: Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    match db.send(Delete { uuid: *userid }).await {
        Ok(Ok(res)) => HttpResponse::Ok().json(res),
        Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong.")
    }
}

#[get("/users")]
async fn get_users(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    match db.send(GetAll{}).await {
        Ok(Ok(users)) => HttpResponse::Ok().json(users),
        _ => HttpResponse::InternalServerError().json("Something went wrong.")
    }
}

async fn ping(req: HttpRequest) -> impl Responder {
    format!("pong")
}

/* APP */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // event logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // pg pool
    let db_url = env::var("DATABASE_URL").expect("Error retrieving database url");
    let pool = get_pool(&db_url);
    
    // spawns 5 actors threads addressable via return value, the sync arbiter is responsible for assigning messages to free actors
    let db_addrs = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: db_addrs.clone() }))
            .route("/ping", web::get().to(ping))
            .service(create_user)
            .service(patch_user)
            .service(delete_user)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}