use actix_web::{App, web, post, Result, HttpServer, HttpResponse};
use std::thread;
use serde::Deserialize;

mod utils;

#[derive(Deserialize)]
struct SettingsRequest {
    tdp: u32,
}

#[post("/update_settings")]
async fn set_tdp_handler(body: web::Json<SettingsRequest>) -> Result<HttpResponse> {
    //let tdp = body.tdp;
    //utils::set_tdp(tdp)
    //    .map(|_| HttpResponse::NoContent().finish())
    //    .map_err(|err| actix_web::error::ErrorBadRequest(err))
    Ok(HttpResponse::NoContent().finish())
}

pub fn start_server() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        let _ = actix_web::rt::System::new().block_on(async {
            let _three = HttpServer::new(|| App::new().service(set_tdp_handler))
                .bind(("127.0.0.1", 1338))
                .expect("Failed to bind server to address")
                .run();

            println!("Server started!");

            _three.await
        });
    })
}