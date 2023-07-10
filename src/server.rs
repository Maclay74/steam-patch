use actix_web::{App, web, post, Result, HttpServer, HttpResponse};
use std::thread;
use serde::Deserialize;
use actix_cors::Cors;

mod utils;

#[derive(Deserialize)]
struct SettingsRequest {
    global: Option<GlobalConfig>,
    per_app: Option<PerAppConfig>,
}

#[derive(Deserialize)]
struct GlobalConfig {
    debug_force_hdr_support: Option<bool>,
    diagnostic_update_rate: Option<i32>,
    force_hdr_10pq_output_debug: Option<bool>,
    force_hdr_wide_gammut_for_sdr: Option<bool>,
    graphics_profiling_service_state: Option<i32>,
    hdr_on_sdr_tonemap_operator: Option<i32>,
    is_advanced_settings_enabled: Option<bool>,
    is_show_perf_overlay_over_steam_enabled: Option<bool>,
    perf_overlay_level: Option<i32>,
    perf_overlay_service_state: Option<i32>,
    sdr_to_hdr_brightness: Option<i32>,
    system_trace_service_state: Option<i32>,
}

#[derive(Deserialize)]
struct PerAppConfig {
    cpu_governor: Option<i32>,
    cpu_governor_manual_mhz: Option<i32>,
    display_external_refresh_manual_hz: Option<i32>,
    display_refresh_manual_hz: Option<i32>,
    force_composite: Option<bool>,
    fps_limit: Option<i32>,
    fps_limit_external: Option<i32>,
    fsr_sharpness: Option<i32>,
    is_composite_debug_enabled: Option<bool>,
    is_dynamic_refresh_rate_enabled: Option<bool>,
    is_fps_limit_enabled: Option<bool>,
    is_low_latency_mode_enabled: Option<bool>,
    is_tdp_limit_enabled: Option<bool>,
    is_tearing_enabled: Option<bool>,
    is_variable_resolution_enabled: Option<bool>,
    is_vrr_enabled: Option<bool>,
    nis_sharpness: Option<i32>,
    split_scaling_filter: Option<i32>,
    split_scaling_scaler: Option<i32>,
    tdp_limit: Option<i32>,
    use_dynamic_refresh_rate_in_steam: Option<bool>,
}

#[post("/update_settings")]
async fn update_settings(settings: web::Json<SettingsRequest>) -> Result<HttpResponse> {
    if let Some(per_app) = &settings.per_app {

        // TDP changes
        if let Some(tdp) = per_app.tdp_limit {
            utils::set_tdp(tdp)
                .map_err(|err| actix_web::error::ErrorBadRequest(err))?;
        }
    }
    Ok(HttpResponse::NoContent().finish())
}

pub fn start_server() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        let _ = actix_web::rt::System::new().block_on(async {
            let _three = HttpServer::new(|| App::new()
                .wrap(
                    Cors::permissive() // enables CORS for all origins
                )
                .service(set_tdp_handler)
            )
                .bind(("127.0.0.1", 1338))
                .expect("Failed to bind server to address")
                .run();

            println!("Server started!");

            _three.await
        });
    })
}