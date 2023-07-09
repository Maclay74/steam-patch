mod server;

use actix_web::{App, HttpServer};
use server::set_tdp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _three = HttpServer::new(|| App::new()
        .service(set_tdp)
    )
    .bind(("127.0.0.1", 1338))?
    .run();

    println!("Server started!");

    _three.await
}
