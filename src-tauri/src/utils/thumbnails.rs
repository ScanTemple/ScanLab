use base64::{engine::general_purpose, Engine as _};
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat, ImageReader};
use std::io::Cursor;
use std::time::Instant;

// generate thumbnail from source path
pub fn generate_thumbnail_from_path(source_path: &str, size: f32) -> Result<String, String> {
    let start = Instant::now();
    let img = ImageReader::open(source_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;
    let resized = resize(&img, size);
    let bytes = to_webp(&resized).map_err(|e| format!("Failed to encode AVIF: {}", e))?;
    let b64 = general_purpose::STANDARD.encode(bytes);
    println!("Thumb generated in {:?}", start.elapsed());
    Ok(b64)
}

// fn generate_thumbnail_from_stage(stage: &ProcessingStage) -> Result<String, String> {}

pub fn resize(img: &DynamicImage, to: f32) -> DynamicImage {
    let (w, h) = img.dimensions();
    let scale = to / w.max(h) as f32;
    let new_w = (w as f32 * scale).round() as u32;
    let new_h = (h as f32 * scale).round() as u32;
    img.resize_exact(new_w, new_h, FilterType::Nearest)
}

fn to_webp(img: &DynamicImage) -> Result<Vec<u8>, image::ImageError> {
    let mut buffer: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::WebP)?;
    Ok(buffer)
}
