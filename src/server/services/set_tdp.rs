use actix_web::{web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(set_tdp);
}

#[actix_web::get("/set_tdp")]
async fn set_tdp() -> impl Responder {
    println!("Set TDP!");
    HttpResponse::Ok().body("Set TDP!")
}