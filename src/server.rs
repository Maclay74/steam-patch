use actix_web::{App, web, get,  Result, HttpServer};
use std::thread;

mod utils;

#[get("/set_tdp/{tdp}")]
pub(crate) async fn set_tdp_handler(path: web::Path<u32>) -> Result<String> {
    let tdp = path.into_inner();
    println!("Set TDP!! {}", tdp);
    Ok(utils::set_tdp(tdp).unwrap().to_string())
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