#[macro_use]
extern crate log;

use app::project::Project;
use std::env;
use std::sync::{Arc, Mutex};
use tauri::{Builder, Manager};

mod app;
mod commands;
pub mod stages;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // config join bundle identifier
    let config_dir = dirs::config_dir()
        .expect("missing config dir")
        .join("ScanLab");

    // https://github.com/tauri-apps/tauri/issues/9394
    // TODO: check for GPU
    // #[cfg(target_os = "linux")]
    // {
    //     let dri_exists = std::path::Path::new("/dev/dri").exists();
    //     let wayland_display = env::var("WAYLAND_DISPLAY").is_ok();
    //     let xdg_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();

    //     // info!(
    //     //     "DRI exists: {dri_exists}, Wayland display variable: {wayland_display}, XDG session type: {xdg_type}",
    //     // );

    //     // Disable dmabuf renderer for wayland sessions
    //     // This is a workaround for issues with dmabuf renderer in some environments
    //     if wayland_display && dri_exists && (xdg_type == "wayland" || xdg_type == "x11") {
    //         env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    //         warn!("Disabled dmabuf renderer for Wayland/X11 session");
    //     }
    // }

    Builder::default()
        .manage(commands::AppState {
            project: Arc::new(Mutex::new(Project::new())),
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_stage,
            commands::get_stage,
            commands::list_stages,
            commands::show_image,
            commands::generate_thumbnail_from_path,
            commands::get_cpus,
            commands::generate_random_name,
            commands::create_project,
            commands::create_temp_project,
            commands::load_project,
            commands::save_project,
            commands::drop_images,
            commands::open_images,
        ])
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Debug)
                        .format(|out, message, record| {
                            out.finish(format_args!(
                                "[{} {}] {}",
                                record.level(),
                                record.target(),
                                message
                            ))
                        })
                        .build(),
                )?;
            }

            let app_handle = app.handle();

            std::fs::create_dir_all(
                app_handle
                    .path()
                    .app_cache_dir()
                    .expect("missing app cache dir"),
            )
            .expect("failed to create cache dir");

            std::fs::create_dir_all(
                app_handle
                    .path()
                    .app_data_dir()
                    .expect("missing app data dir"),
            )
            .expect("failed to create data dir");
            std::fs::create_dir_all(&config_dir).expect("failed to create config dir");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
