use actix_web::{get, web, Responder};

#[get("/tags")]
pub async fn get_tags(db: web::Data<crate::db::MySQLClient>) -> impl Responder {
    let m = db
        .transaction(|mut tx| Ok(crate::dao::get_tags(&mut tx)?))
        .unwrap();
    web::Json(m)
}
