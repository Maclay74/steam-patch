use actix_web::{web, get,  Result};

#[get("/set_tdp/{tdp}")]
pub(crate) async fn set_tdp(path: web::Path<u32>) -> Result<String> {
    let tdp = path.into_inner();
    println!("Set TDP!! {}", tdp);
    Ok(format!("You updated TDP to {}", tdp))
}