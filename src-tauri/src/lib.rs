use app::project::{ProcessingPipeline, Project};
use std::sync::{Arc, Mutex};
use tauri::{Builder, Manager};

mod app;
mod commands;
mod pipeline;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // config join bundle identifier
    let config_dir = dirs::config_dir()
        .expect("missing config dir")
        .join("ScanLab");

    Builder::default()
        .manage(commands::AppState {
            project: Arc::new(Mutex::new(Project {
                pipeline: ProcessingPipeline { steps: vec![] },
                file_path: None,
            })),
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_stage,
            commands::show_image,
            commands::generate_thumbnail_from_path,
            commands::get_cpus,
            commands::generate_random_name,
            commands::create_project,
            commands::load_project,
            commands::save_project,
        ])
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let app_handle = app.handle();

            std::fs::create_dir_all(
                &app_handle
                    .path()
                    .app_cache_dir()
                    .expect("missing app cache dir"),
            )
            .expect("failed to create cache dir");
            std::fs::create_dir_all(&config_dir).expect("failed to create config dir");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
