use hyper::service::{make_service_fn, service_fn};
use hyper::Request;
use hyper::{body, Body, Method, Response, Server};
use serde::Deserialize;
use std::convert::Infallible;

use crate::devices::create_device;

#[derive(Deserialize)]
pub struct SettingsRequest {
    pub per_app: Option<PerAppConfig>,
}

#[derive(Deserialize)]
pub struct PerAppConfig {
    pub tdp_limit: Option<i8>,
}

async fn update_settings(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Update settings");

    // Convert the request body into bytes
    let bytes = body::to_bytes(req.into_body())
        .await
        .map_err(|_| Response::new(Body::from("Internal server error")))
        .unwrap();

    // Parse the bytes into a SettingsRequest
    let settings_request: SettingsRequest = serde_json::from_slice(&bytes)
        .map_err(|_| Response::new(Body::from("Failed to deserialize request body")))
        .unwrap();

    // Handle your request here, using the deserialized settings_request
    if let Some(device) = create_device() {
        device.update_settings(settings_request);
    }

    Ok(Response::new(Body::from("Settings updated")))
}

async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path(); // Get the path of the request

    match (req.method(), path) {
        (&Method::POST, "/update_settings") => update_settings(req).await,
        _ => Ok(Response::new(Body::empty())),
    }
}

pub async fn run() {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router)) });

    let addr = ([127, 0, 0, 1], 1338).into();

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
