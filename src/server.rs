use actix_web::{App, web, post, Result, HttpServer, HttpResponse};
use std::thread;
use serde::Deserialize;

mod utils;

#[derive(Deserialize)]
struct SettingsRequest {
    global: GlobalConfig,
    per_app: PerAppConfig,
}

#[derive(Deserialize)]
struct GlobalConfig {
    debug_force_hdr_support: bool,
    diagnostic_update_rate: i32,
    force_hdr_10pq_output_debug: bool,
    force_hdr_wide_gammut_for_sdr: bool,
    graphics_profiling_service_state: i32,
    hdr_on_sdr_tonemap_operator: i32,
    is_advanced_settings_enabled: bool,
    is_show_perf_overlay_over_steam_enabled: bool,
    perf_overlay_level: i32,
    perf_overlay_service_state: i32,
    sdr_to_hdr_brightness: i32,
    system_trace_service_state: i32,
}

#[derive(Deserialize)]
struct PerAppConfig {
    cpu_governor: i32,
    cpu_governor_manual_mhz: i32,
    display_external_refresh_manual_hz: i32,
    display_refresh_manual_hz: i32,
    force_composite: bool,
    fps_limit: i32,
    fps_limit_external: i32,
    fsr_sharpness: i32,
    is_composite_debug_enabled: bool,
    is_dynamic_refresh_rate_enabled: bool,
    is_fps_limit_enabled: bool,
    is_low_latency_mode_enabled: bool,
    is_tdp_limit_enabled: bool,
    is_tearing_enabled: bool,
    is_variable_resolution_enabled: bool,
    is_vrr_enabled: bool,
    nis_sharpness: i32,
    split_scaling_filter: i32,
    split_scaling_scaler: i32,
    tdp_limit: i32,
    use_dynamic_refresh_rate_in_steam: bool,
}

#[post("/update_settings")]
async fn set_tdp_handler(settings: web::Json<SettingsRequest>) -> Result<HttpResponse> {
    let tdp = settings.per_app.tdp_limit;
    utils::set_tdp(tdp)
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(|err| actix_web::error::ErrorBadRequest(err))
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