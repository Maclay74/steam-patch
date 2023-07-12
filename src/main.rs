mod server;
mod steam;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut threads = Vec::new();

    threads.push(server::start_server());
    steam::patch_steam();

    for thread in threads {
        thread.join().unwrap();
    }

    Ok(())
}