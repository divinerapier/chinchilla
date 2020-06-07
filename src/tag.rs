use actix_web::{get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::StreamExt;
use mysql::prelude::*;
use mysql::*;
use serde_derive::Deserialize;

#[get("/tags")]
pub async fn get_tags(db: web::Data<crate::db::MySQLClient>) -> String {
    let m = db
        .transaction(|mut tx| Ok(crate::dao::get_tags(&mut tx)?))
        .unwrap();
    format!("{:?}", m)
}
