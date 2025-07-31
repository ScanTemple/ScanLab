use base64::{engine::general_purpose, Engine as _};
use infer;
use std::fs::File;
use std::io::Read;

pub mod thumbnails;

pub fn show_image(source_path: &str) -> Result<String, String> {
    let mut file = File::open(source_path).expect("Failed to open image file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read image file");

    let mime_type = if let Some(kind) = infer::get(&buffer) {
        kind.mime_type()
    } else {
        "application/octet-stream"
    };

    let encoded = general_purpose::STANDARD.encode(&buffer);

    return Ok(format!("data:{};base64,{}", mime_type, encoded));
}
