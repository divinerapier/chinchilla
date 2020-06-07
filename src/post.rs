use actix_web::{get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::StreamExt;
use mysql::prelude::*;
use mysql::*;
use serde_derive::Deserialize;

struct PostTag<'a> {
    post_uuid: &'a str,
    tag: &'a str,
}

#[post("/post")]
pub async fn create_post(
    req: web::Json<crate::types::CreatePostRequest>,
    db: web::Data<crate::db::MySQLClient>,
    coll: crate::types::MongoDBCollection,
) -> String {
    let u = crate::uuid::v4();
    let doc = mongodb::bson::doc! {
    "uuid": u.clone() ,
    "content": req.content.clone(),
    };
    coll.insert_one(doc, None).await.unwrap();

    db.transaction(|mut tx| {
        crate::dao::create_post(&mut tx, &u, &req.title, &req.link_name)?;
        crate::dao::create_post_tag(&mut tx, &u, &req.title)?;
        Ok(())
    })
        .unwrap();

    format!("{}", u)
}
