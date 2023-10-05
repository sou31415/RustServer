use crate::structure::{Chat, Info};
use actix_web::{
    delete, get, post, put, web, App, HttpResponse, HttpServer, Responder, ResponseError,
};
use futures_util::{stream::TryStreamExt, TryFutureExt};
use mongodb::bson::{doc, oid::ObjectId, to_document};
use mongodb::options::{ClientOptions, FindOptions, ReturnDocument};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

#[post("/login/register")]
pub async fn regist(data: web::Json<Info>) -> impl Responder {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .expect("Error : (Hint server/src/main.rs:26");
    let client = Client::with_options(client_options).expect("Error : (Hint server/src/main.rs:27");
    let datas = data.0.clone();
    let db = client.database("server");
    let collection = db.collection::<Info>("info");
    let infos = vec![datas];
    collection
        .insert_many(infos, None)
        .await
        .expect("Error : (Hint server/src/main.rs:35");
    println!("Ok");
    format!("Accepted")
}
//非同期処理が必要なためasync fnでmainを定義する

#[post("/system/questions")]
pub async fn posting(data: web::Json<Chat>) -> impl Responder {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .expect("DataBaseError:34");
    let client = Client::with_options(client_options).expect("DataBaseError:35");
    let datas = data.0.clone();
    let db = client.database("server");
    let collection = db.collection::<Chat>("chats");
    let chats = vec![datas];
    collection
        .insert_many(chats, None)
        .await
        .expect("DataBaseError:49");
    println!("Ok");
    format!(" submitted {:?}", data.0)
}

#[post("/login/cert")]
pub async fn auth(data: web::Json<Info>) -> impl Responder {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .expect("DataBaseError");
    let client = Client::with_options(client_options).expect("Error : (Hint server/src/main.rs:27");
    let datas = data.0.clone();
    let filter = doc! {"user":datas.clone().user};
    let db = client.database("server");
    let col = db.collection::<Info>("info");
    let find_options = FindOptions::builder().build();
    let mut cursor = col
        .find(filter, find_options)
        .await
        .expect("DataBaseError:58");
    while let Some(book) = cursor.try_next().await.expect("DataBaseError:60") {
        if book.pass == datas.pass {
            return format!("Ok");
        } else {
            return format!("Err");
        }
    }
    return format!("Err");
}
