use base64::{engine::general_purpose, Engine as _};
use fast_image_resize::images::Image;
use fast_image_resize::{FilterType, IntoImageView, ResizeAlg, ResizeOptions, Resizer};
use image::codecs::png::PngEncoder;
use image::{DynamicImage, GenericImageView, ImageEncoder, ImageReader};
use std::io::BufWriter;
use std::time::Instant;

// generate thumbnail from source path
pub fn generate_thumbnail_from_path(source_path: &str, size: f32) -> Result<String, String> {
    let start = Instant::now();

    let img = ImageReader::open(source_path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    println!("Image decoded in {:?}", start.elapsed());
    let thumbstart = Instant::now();

    let resized_buf = resize(&img, size);
    let b64 = general_purpose::STANDARD.encode(resized_buf.into_inner().unwrap());

    println!("Thumb generated in {:?}", thumbstart.elapsed());

    Ok(format!("data:image/png;base64,{}", b64))
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
    resizer
        .resize(img, &mut dst_image, &resize_options)
        .unwrap();

    let mut result_buf = BufWriter::new(Vec::new());
    PngEncoder::new(&mut result_buf)
        .write_image(dst_image.buffer(), new_w, new_h, img.color().into())
        .unwrap();

    result_buf
}
