use actix_web::{get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use mongodb::bson;
use mongodb::options::FindOptions;

use crate::error::Error;

#[post("/post")]
pub async fn create_post(
    req: web::Json<crate::types::CreatePostRequest>,
    db: web::Data<crate::db::MySQLClient>,
    coll: web::Data<mongodb::Collection>,
) -> impl Responder {
    let u = crate::uuid::v4();
    let doc = mongodb::bson::doc! {
    "uuid": u.clone() ,
    "content": req.content.clone(),
    };
    coll.insert_one(doc, None).await.unwrap();
    match db.transaction(|mut tx| {
        crate::dao::create_post(&mut tx, &u, &req.title, &req.link_name)?;
        crate::dao::create_post_tag(&mut tx, &u, &req.tags)?;
        Ok(())
    }) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(Error::R2D2(e)) => HttpResponse::InternalServerError().body(format!("r2d2: {:?}", e)),
        Err(Error::MySQL(mysql::error::Error::MySqlError(e))) => {
            if e.code == 1062 {
                HttpResponse::BadRequest().body(format!("duplicate link_name"))
            } else {
                HttpResponse::InternalServerError().body(format!("mysql error: {:?}", e))
            }
        }
        e @ Err(Error::MySQL(_)) => {
            HttpResponse::InternalServerError().body(format!("mysql other: {:?}", e))
        }
    }
}

#[get("/posts")]
pub async fn get_post_list(
    db: web::Data<crate::db::MySQLClient>,
    web::Query(query): web::Query<crate::types::GetPostListRequestOptions>,
) -> impl Responder {
    let posts = db.transaction(|mut tx| {
        Ok(crate::dao::get_post_list(
            &mut tx,
            &query.offset,
            &query.limit,
            &query.tag,
        )?)
    });
    println!("get posts done");
    match posts {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(Error::R2D2(e)) => HttpResponse::InternalServerError().body(format!("r2d2: {:?}", e)),
        Err(Error::MySQL(e)) => HttpResponse::InternalServerError().body(format!("mysql: {:?}", e)),
    }
}

#[get("/post/{link_name}")]
pub async fn get_post(
    link_name: web::Path<String>,
    db: web::Data<crate::db::MySQLClient>,
    coll: web::Data<mongodb::Collection>,
) -> impl Responder {
    let mut conn = db.get().unwrap();
    let post: Option<crate::types::Post> =
        crate::dao::get_post_by_link_name(&mut conn, &link_name).unwrap();
    if post.is_none() {
        return HttpResponse::NotFound().finish();
    }
    let mut post: crate::types::Post = post.unwrap();
    let filter = bson::doc! {"uuid": &post.uuid };
    let find_options = FindOptions::builder().limit(1).build();
    let mut cursor: mongodb::Cursor = coll.find(filter, find_options).await.unwrap();
    match cursor.next().await {
        Some(doc) => match doc {
            Ok(doc) => {
                let doc: mongodb::bson::Document = doc;
                match doc.get("content").and_then(bson::Bson::as_str) {
                    Some(content) => {
                        post.content = Some(content.to_string());
                        HttpResponse::Ok().json(post)
                    }
                    None => HttpResponse::InternalServerError().body(format!("internal error")),
                }
            }
            Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        },
        None => HttpResponse::InternalServerError().body(format!("internal error")),
    }
}
