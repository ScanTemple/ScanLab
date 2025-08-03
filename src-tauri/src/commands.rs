use crate::app::project::{ProcessingStage, Project};
use crate::utils;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{command, State};

pub struct AppState {
    pub project: Arc<Mutex<Project>>,
}

#[command(async)]
pub fn add_stage(state: State<'_, AppState>, stage: ProcessingStage) {
    let mut project = state.project.lock().unwrap();
    project.pipeline.steps.push(stage);
}

// create project
#[command(async)]
pub fn create_project(state: State<'_, AppState>, name: String, dir: String) -> Result<(), String> {
    let mut project = state.project.lock().unwrap();
    *project = Project::new();
    project.file_path = Some(PathBuf::from(dir).join(format!("{name}.ScanLab")));

    project
        .save()
        .map_err(|e| format!("Failed to create project: {e}"))?;

    info!("Project created at {:?}", project.file_path);

    Ok(())
}

// load project
#[command(async)]
pub fn load_project(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let mut project = state.project.lock().unwrap();
    *project = Project::load_from_file(PathBuf::from(path))
        .map_err(|e| format!("Failed to load project: {e}"))?;

    info!("Project loaded from {:?}", project.file_path);

    Ok(())
}

// save project
#[command(async)]
pub fn save_project(state: State<'_, AppState>) -> Result<(), String> {
    let mut project = state.project.lock().unwrap();
    project.save().unwrap();
    Ok(())
}

#[command(async)]
pub fn generate_thumbnail_from_path(
    app_handle: tauri::AppHandle,
    source_path: String,
    size: f32,
) -> Result<String, String> {
    utils::thumbnails::generate_thumbnail_from_path(&source_path, size, &app_handle)
}

#[command(async)]
pub fn show_image(source_path: String) -> Result<String, String> {
    utils::show_image(&source_path)
}

#[command(async)]
pub fn get_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1)
}

#[command(async)]
pub fn generate_random_name() -> String {
    names::Generator::default()
        .next()
        .unwrap_or_else(|| "default-name".to_string())
}
