use hyper::http::HeaderValue;
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
#[derive(Debug)]
pub struct PerAppConfig {
    pub tdp_limit: Option<i8>,
    pub gpu_performance_manual_mhz: Option<i16>,
    pub is_tdp_limit_enabled: Option<bool>
}

async fn update_settings(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Received request to update settings.");

    // Convert the request body into bytes
    let bytes = match body::to_bytes(req.into_body()).await {
        Ok(b) => b,
        Err(e) => {
            println!("Error converting request body to bytes: {}", e);
            return Ok(Response::new(Body::from("Internal server error")));
        },
    };

    // Parse the bytes into a SettingsRequest
    let settings_request: SettingsRequest = match serde_json::from_slice(&bytes) {
        Ok(req) => req,
        Err(e) => {
            println!("Error deserializing request body: {}", e);
            return Ok(Response::new(Body::from("Failed to deserialize request body")));
        },
    };

    if let Some(device) = create_device() {
        println!("Device created, updating settings.");
        device.update_settings(settings_request);
    } else {
        println!("Failed to create device.");
    }

    println!("Settings updated successfully.");
    Ok(Response::new(Body::from("Settings updated")))
}


fn set_cors_headers(mut response: Response<Body>) -> Response<Body> {
    let headers = response.headers_mut();

    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("*"),
    );

    response
}

async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path(); // Get the path of the request
    println!("Routing request to {}", path);

    let response = match (req.method(), path) {
        (&Method::POST, "/update_settings") => {
            println!("Handling POST request to /update_settings");
            update_settings(req).await
        },
        _ => {
            println!("No route found for {} {}", req.method(), path);
            Ok(Response::new(Body::from("404 Not Found")))
        },
    };

    println!("Request routed, setting CORS headers.");
    Ok(set_cors_headers(response?))
}

pub async fn run() {
    let make_svc = make_service_fn(|_conn| async {
        println!("Connection established, creating service.");
        Ok::<_, Infallible>(service_fn(router))
    });

    let addr = ([127, 0, 0, 1], 1338).into();
    println!("Attempting to bind server to address: {:?}", addr);

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server is running on http://{}", addr);

    if let Err(e) = server.await {
        println!("Server error: {}", e);
    }
}
