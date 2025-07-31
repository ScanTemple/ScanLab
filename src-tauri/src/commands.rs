use crate::utils::thumbnails;
use tauri::command;

#[command(async)]
pub fn generate_thumbnail_from_path(source_path: String, size: f32) -> Result<String, String> {
    thumbnails::generate_thumbnail_from_path(&source_path, size)
}

#[command(async)]
pub fn get_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1)
}
