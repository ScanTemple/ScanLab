use crate::app::project::Project;
use crate::stages::ProcessingStage;
use crate::utils;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{command, Manager, State};
use tauri_plugin_dialog::DialogExt;

pub struct AppState {
    pub project: Arc<Mutex<Project>>,
}

#[command(async)]
pub fn add_stage(state: State<'_, AppState>, stage: String) -> Result<(), String> {
    let mut project = state.project.lock().unwrap();
    project
        .stages
        .push(ProcessingStage::new_stage(&stage).map_err(|e| format!("Failed to add stage: {e}"))?);
    Ok(())
}

#[command(async)]
pub fn get_stage(state: State<'_, AppState>, index: usize) -> Result<ProcessingStage, String> {
    let project = state.project.lock().unwrap();
    project
        .stages
        .get(index)
        .cloned()
        .ok_or_else(|| format!("No stage found at index {index}"))
}

#[command(async)]
pub fn list_stages(state: State<'_, AppState>) -> Result<Vec<ProcessingStage>, String> {
    let project = state.project.lock().unwrap();
    Ok(project.stages.clone())
}

// create project
#[command(async)]
pub fn create_project(state: State<'_, AppState>, name: String, dir: String) -> Result<(), String> {
    let mut project = state.project.lock().unwrap();
    *project = Project::new();
    project.file_path = Some(PathBuf::from(dir).join(format!("{name}.ScanLab")));
    project
        .stages
        .push(ProcessingStage::new_stage("open").unwrap());

    project
        .save()
        .map_err(|e| format!("Failed to create project: {e}"))?;

    info!("Project created at {:?}", project.file_path);

    Ok(())
}

#[command(async)]
pub fn create_temp_project(
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("missing app data dir");

    let mut project = state.project.lock().unwrap();
    *project = Project::new();
    project.file_path = Some(app_data_dir.join("temp_project.ScanLab"));

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

#[command(async)]
pub fn drop_images(
    state: State<'_, AppState>,
    paths: Vec<String>,
    position: Option<usize>,
) -> Result<(), String> {
    let mut project = state.project.lock().unwrap();

    if let Some(stage) = project.stages.first_mut() {
        if let ProcessingStage::Open(ref mut open_stage) = stage {
            for (ix, path) in paths.iter().enumerate() {
                if let Some(pos) = position {
                    open_stage.add_image_at(path.to_string(), pos + ix);
                } else {
                    open_stage.add_image(path.to_string());
                }
            }
        } else {
            return Err("Invalid stage type".into());
        }
    } else {
        return Err("No stages available".into());
    }

    Ok(())
}

#[command(async)]
pub fn open_images(
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
    position: Option<usize>,
) -> Result<(), String> {
    // Open file dialog for multiple image selection
    let files = app_handle
        .dialog()
        .file()
        .add_filter("Scans", &["png", "jpg", "jpeg", "bmp", "tiff", "tif"])
        .set_title("Select Scans")
        .blocking_pick_files();

    let files = match files {
        Some(files) => files,
        None => return Err("No files selected".to_string()),
    };

    let mut project = state.project.lock().unwrap();

    if let Some(stage) = project.stages.first_mut() {
        if let ProcessingStage::Open(ref mut open_stage) = stage {
            for (ix, file) in files.iter().enumerate() {
                if let Some(pos) = position {
                    open_stage.add_image_at(file.to_string(), pos + ix);
                } else {
                    open_stage.add_image(file.to_string());
                }
            }
        } else {
            return Err("Invalid stage type".into());
        }
    } else {
        return Err("No stages available".into());
    }

    info!("Images opened: {files:?}");

    Ok(())
}

// #[command(async)]
// pub fn open_images(
//     state: State<'_, AppState>,
//     paths: Vec<String>,
//     position: usize,
// ) -> Result<(), String> {
//     let mut project = state.project.lock().unwrap();

//     match project.stages[0] {
//         ProcessingStage::Open(ref stage) => {
//             for path in paths {
//                 stage.images.push(ImageInfo::new(path));
//             }
//         }
//         _ => return Err("Invalid stage type".into()),
//     }

//     Ok(())
// }
