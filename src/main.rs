#![allow(unused_imports)]
pub mod crypto;
pub mod func;
pub mod structure;
use actix_web::{
    delete, get, post, put, web, App, HttpResponse, HttpServer, Responder, ResponseError,
};
use func::{auth, posting, regist};
use mongodb::bson::{doc, oid::ObjectId, to_document};
use mongodb::options::{ClientOptions, FindOptions, ReturnDocument};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

const ADDRESS: &str = "10.72.1.25:8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(regist).service(posting).service(auth))
        .bind(ADDRESS)?
        .run()
        .await
}
