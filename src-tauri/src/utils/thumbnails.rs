use base64::{engine::general_purpose, Engine as _};
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat, ImageReader};
use std::io::Cursor;

// generate thumbnail from source path
pub fn generate_thumbnail_from_path(source_path: &str, size: f32) -> Result<String, String> {
    println!("started thumb gen");
    let img = ImageReader::open(source_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;
    println!("image opened and decoded");
    let resized = resize(&img, size);
    println!("image resized to: {}x{}", resized.width(), resized.height());
    let bytes = to_avif(&resized).map_err(|e| format!("Failed to encode AVIF: {}", e))?;
    println!("image encoded to AVIF");
    let b64 = general_purpose::STANDARD.encode(bytes);
    println!("image encoded to base64");
    println!("thumbnail generation completed");
    Ok(b64)
}

// fn generate_thumbnail_from_stage(stage: &ProcessingStage) -> Result<String, String> {}

pub fn resize(img: &DynamicImage, to: f32) -> DynamicImage {
    let (w, h) = img.dimensions();
    let scale = to / w.max(h) as f32;
    let new_w = (w as f32 * scale).round() as u32;
    let new_h = (h as f32 * scale).round() as u32;
    img.resize_exact(new_w, new_h, FilterType::Lanczos3)
}

fn to_avif(img: &DynamicImage) -> Result<Vec<u8>, image::ImageError> {
    let mut buffer: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
    Ok(buffer)
}
