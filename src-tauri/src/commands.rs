use crate::utils::thumbnails;
use tauri::command;

#[command(async)]
pub fn generate_thumbnail_from_path(source_path: String, size: f32) -> Result<String, String> {
    thumbnails::generate_thumbnail_from_path(&source_path, size)
}
