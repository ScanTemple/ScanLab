use crate::utils;
use tauri::command;

#[command(async)]
pub fn generate_thumbnail_from_path(source_path: String, size: f32) -> Result<String, String> {
    utils::thumbnails::generate_thumbnail_from_path(&source_path, size)
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
