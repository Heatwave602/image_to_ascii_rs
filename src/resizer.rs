use std::error::Error;

use image::{DynamicImage, ImageBuffer, ImageReader};
use image::imageops::FilterType;
use image::Rgb;

const BASE_WIDTH: u32 = 80;

pub fn downscale(
  path: &str,
) -> Result<DynamicImage, Box<dyn Error>> {
  let image = ImageReader::open(path)?.decode()?.to_rgb16();
  let new_height = compute_new_height(&image);

  let ds_image = DynamicImage::resize(
    &DynamicImage::ImageRgb16(image),
    BASE_WIDTH,
    new_height,
    FilterType::Lanczos3,
  );

  Ok(ds_image)
}

fn compute_new_height(
  image: &ImageBuffer<Rgb<u16>, Vec<u16>>,
) -> u32 {
  let f32_width   = image.width() as f32;
  let f32_height  = image.height() as f32;
  assert!(f32_width != 0. && f32_height != 0.);

  let ar = f32_width/f32_height;
  ((BASE_WIDTH as f32 / ar) / 2.) as u32
}