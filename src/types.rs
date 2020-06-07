use actix_web::{get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::StreamExt;
use mysql::prelude::*;
use serde_derive::Deserialize;

pub type MySQLClient = web::Data<r2d2::Pool<r2d2_mysql::MysqlConnectionManager>>;
pub type MongoDBCollection = web::Data<mongodb::Collection>;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub link_name: String,
    pub tags: String,
    pub content: String,
}
