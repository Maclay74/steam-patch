use actix_web::{App, web, post, Result, HttpServer, HttpResponse};
use std::thread;
use serde::Deserialize;
use actix_cors::Cors;
use super::devices::{create_device};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SettingsRequest {
    pub global: Option<GlobalConfig>,
    pub per_app: Option<PerAppConfig>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct GlobalConfig {
    debug_force_hdr_support: Option<bool>,
    diagnostic_update_rate: Option<i64>,
    force_hdr_10pq_output_debug: Option<bool>,
    force_hdr_wide_gammut_for_sdr: Option<bool>,
    graphics_profiling_service_state: Option<i64>,
    hdr_on_sdr_tonemap_operator: Option<i64>,
    is_advanced_settings_enabled: Option<bool>,
    is_show_perf_overlay_over_steam_enabled: Option<bool>,
    perf_overlay_level: Option<i64>,
    perf_overlay_service_state: Option<i64>,
    sdr_to_hdr_brightness: Option<f64>,
    system_trace_service_state: Option<i64>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct PerAppConfig {
    cpu_governor: Option<i64>,
    cpu_governor_manual_mhz: Option<i64>,
    display_external_refresh_manual_hz: Option<i64>,
    display_refresh_manual_hz: Option<i64>,
    force_composite: Option<bool>,
    fps_limit: Option<i64>,
    fps_limit_external: Option<i64>,
    fsr_sharpness: Option<i64>,
    is_composite_debug_enabled: Option<bool>,
    is_dynamic_refresh_rate_enabled: Option<bool>,
    is_fps_limit_enabled: Option<bool>,
    is_low_latency_mode_enabled: Option<bool>,
    is_tdp_limit_enabled: Option<bool>,
    is_tearing_enabled: Option<bool>,
    is_variable_resolution_enabled: Option<bool>,
    is_vrr_enabled: Option<bool>,
    nis_sharpness: Option<i64>,
    split_scaling_filter: Option<i64>,
    split_scaling_scaler: Option<i64>,
    pub tdp_limit: Option<i8>,
    use_dynamic_refresh_rate_in_steam: Option<bool>,
}

#[post("/update_settings")]
async fn update_settings(settings: web::Json<SettingsRequest>) -> Result<HttpResponse> {

    println!("Request to update settings");

    if let Some(device) = create_device() {
        device.update_settings(settings.into_inner());
    }

     // I need to pass here settings but as
    Ok(HttpResponse::NoContent().finish())
}

pub fn start_server() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        let _ = actix_web::rt::System::new().block_on(async {
            let _three = HttpServer::new(|| App::new()
                .wrap(
                    Cors::permissive() // enables CORS for all origins
                )
                .service(update_settings)
            )
                .bind(("127.0.0.1", 1338))
                .expect("Failed to bind server to address")
                .run();

            println!("Server started!");

            _three.await
        });
    })
}