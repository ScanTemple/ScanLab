use pipeline::{ProcessingPipeline, ProcessingStage};
use std::sync::{Arc, Mutex};
use tauri::{command, Builder, State, Manager};

mod commands;
mod pipeline;
mod utils;

pub struct AppState {
    pub pipeline: Arc<Mutex<ProcessingPipeline>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // config join bundle identifier
    let config_dir = dirs::config_dir()
        .expect("missing config dir")
        .join("ScanLab");

    Builder::default()
        .manage(AppState {
            pipeline: Arc::new(Mutex::new(ProcessingPipeline { steps: vec![] })),
        })
        .invoke_handler(tauri::generate_handler![
            add_stage,
            commands::show_image,
            commands::generate_thumbnail_from_path,
            commands::get_cpus,
            commands::generate_random_name,
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
            
            std::fs::create_dir_all(&app_handle.path().app_cache_dir().expect("missing app cache dir")).expect("failed to create cache dir");
            std::fs::create_dir_all(&config_dir).expect("failed to create config dir");
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn add_stage(state: State<'_, AppState>, stage: ProcessingStage) {
    let mut pipeline = state.pipeline.lock().unwrap();
    pipeline.steps.push(stage);
}
