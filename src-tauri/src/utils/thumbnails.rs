use base64::{engine::general_purpose, Engine as _};
use fast_image_resize::images::Image;
use fast_image_resize::{FilterType, IntoImageView, ResizeAlg, ResizeOptions, Resizer};
use image::{DynamicImage, GenericImageView, ImageEncoder, ImageReader};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;
use tauri::Manager;

// generate thumbnail from source path
pub fn generate_thumbnail_from_path(
    source_path: &str,
    size: f32,
    app_handle: &tauri::AppHandle,
) -> Result<String, String> {
    let start = Instant::now();

    let cache_dir = app_handle
        .path()
        .app_cache_dir()
        .map_err(|e| format!("Failed to get cache directory: {e}"))?
        .join("thumbnails");

    fs::create_dir_all(&cache_dir)
        .map_err(|e| format!("Failed to create cache directory: {e}"))?;

    // Generate cache key based on file path, size, and modification time
    let cache_key = generate_cache_key(source_path, size)?;
    let cache_path = cache_dir.join(format!("{cache_key}.avif"));

    // Check if cached thumbnail exists
    if cache_path.exists() {
        match fs::read(&cache_path) {
            Ok(cached_data) => {
                let b64 = general_purpose::STANDARD.encode(&cached_data);
                println!("Thumbnail loaded from cache in {:?}", start.elapsed());
                return Ok(format!("data:image/png;base64,{b64}"));
            }
            Err(e) => {
                println!("Failed to read cached thumbnail: {e}, regenerating");
            }
        }
    }

    let img = ImageReader::open(source_path)
        .map_err(|e| format!("Failed to open image: {e}"))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {e}"))?;

    println!("Image decoded in {:?}", start.elapsed());
    let thumbstart = Instant::now();

    let resized_buf = resize(&img, size);
    let thumbnail_data = resized_buf.into_inner().unwrap();

    // Save thumbnail to cache
    if let Err(e) = fs::write(&cache_path, &thumbnail_data) {
        println!("Failed to save thumbnail to cache: {e}");
    }

    let b64 = general_purpose::STANDARD.encode(&thumbnail_data);

    println!("Thumb generated and cached in {:?}", thumbstart.elapsed());

    Ok(format!("data:image/avif;base64,{b64}"))
}

// fn generate_thumbnail_from_stage(stage: &ProcessingStage) -> Result<String, String> {}

pub fn resize(img: &DynamicImage, to: f32) -> BufWriter<Vec<u8>> {
    let (w, h) = img.dimensions();
    let scale = to / w.max(h) as f32;
    let new_w = (w as f32 * scale).round() as u32;
    let new_h = (h as f32 * scale).round() as u32;
    let mut dst_image = Image::new(new_w, new_h, img.pixel_type().unwrap());

    let resize_options = ResizeOptions::new().resize_alg(ResizeAlg::Convolution(FilterType::Box));
    let mut resizer = Resizer::new();

    #[cfg(target_arch = "x86_64")]
    unsafe {
        resizer.set_cpu_extensions(fast_image_resize::CpuExtensions::Sse4_1);
    }

    resizer
        .resize(img, &mut dst_image, &resize_options)
        .unwrap();

    let mut result_buf = BufWriter::new(Vec::new());
    // PngEncoder::new(&mut result_buf)
    //     .write_image(dst_image.buffer(), new_w, new_h, img.color().into())
    //     .unwrap();

    // image::codecs::webp::WebPEncoder::new_lossless(&mut result_buf)
    //     .encode(dst_image.buffer(), new_w, new_h, img.color().into())
    //     .unwrap();

    image::codecs::avif::AvifEncoder::new_with_speed_quality(&mut result_buf, 10, 50)
        .write_image(dst_image.buffer(), new_w, new_h, img.color().into())
        .unwrap();

    result_buf
}

fn generate_cache_key(source_path: &str, size: f32) -> Result<String, String> {
    let path = Path::new(source_path);

    // Get file modification time
    let metadata = fs::metadata(path).map_err(|e| format!("Failed to get file metadata: {e}"))?;
    let modified = metadata
        .modified()
        .map_err(|e| format!("Failed to get modification time: {e}"))?;

    // Create hash from path, size, and modification time
    let mut hasher = DefaultHasher::new();
    source_path.hash(&mut hasher);
    size.to_bits().hash(&mut hasher);
    modified
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Invalid modification time: {e}"))?
        .as_secs()
        .hash(&mut hasher);

    Ok(hasher.finish().to_string())
}
