use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::StreamExt;
use mysql::prelude::*;
use serde_derive::Deserialize;

mod dao;
mod db;
mod error;
mod post;
mod result;
mod tag;
mod types;
mod uuid;

#[derive(Debug, Deserialize)]
struct Query {
    id: u64,
    null_id: Option<u64>,
}

#[get("/resource1/{name}/index.html")]
async fn index(
    req: HttpRequest,
    name: web::Path<String>,
    db: web::Data<r2d2::Pool<r2d2_mysql::MysqlConnectionManager>>,
    web::Query(query): web::Query<Query>,
) -> String {
    println!("REQ: {:?}", req);
    println!("QUERY: {:?}", query);
    let mut conn = db.get().unwrap();
    let result: Vec<String> = conn.query("SHOW tables").unwrap();
    for line in &result {
        println!("table: {}", line);
    }
    format!("Hello: {}!\r\n", name)
}

async fn index_async(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!\r\n"
}

#[get("/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    let url = "mysql://root:password@192.168.50.124/chinchilla";
    // let opts = mysql::Opts::from_url(&url).unwrap();
    // let builder = mysql::OptsBuilder::from_opts(opts);
    // let manager = r2d2_mysql::MysqlConnectionManager::new(builder);
    // let db_pool = r2d2::Pool::new(manager).unwrap();
    let db_pool = db::MySQLClient::new(url);
    let client = mongodb::Client::with_uri_str("mongodb://root:password@192.168.50.124:27017")
        .await
        .unwrap();
    let db: mongodb::Database = client.database("admin");
    let collection: mongodb::Collection = db.collection("system.users");
    let mut cursor: mongodb::Cursor = collection.find(None, None).await.unwrap();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let document: mongodb::bson::Document = document;
                println!("document: {:?}", document);
            }
            Err(e) => {
                println!("mongo error: {:?}", e);
            }
        }
    }

    let db: mongodb::Database = client.database("chinchilla");
    let collection: mongodb::Collection = db.collection("post");
    {
        // let docs = vec![
        //     mongodb::bson::doc! {"content": "document1"},
        //     mongodb::bson::doc! {"content": "document2"},
        //     mongodb::bson::doc! {"content": "document3"},
        //     mongodb::bson::doc! {"content": "document4"},
        //     mongodb::bson::doc! {"content": "document5"},
        //     mongodb::bson::doc! {"content": "document6"},
        // ];
        // let insert_many_option = collection.insert_many(docs, None).await.unwrap();
        // println!("insert many result: {:?}", insert_many_option);
    }
    let doc = mongodb::bson::doc! {"content:": "single_one"};
    collection.insert_one(doc, None).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(collection.clone())
            .data(db_pool.clone())
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
            // create post
            .service(post::create_post)
            // get tags
            .service(tag::get_tags)
            // get post list
            .service(post::get_post_list)
            // get post
            .service(post::get_post)
            .service(no_params)
            .service(
                web::resource("/resource2/index.html")
                    .wrap(middleware::DefaultHeaders::new().header("X-Version-R2", "0.3"))
                    .default_service(web::route().to(|| HttpResponse::MethodNotAllowed()))
                    .route(web::get().to(index_async)),
            )
            .service(web::resource("/test1.html").to(|| async { "Test\r\n" }))
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}
