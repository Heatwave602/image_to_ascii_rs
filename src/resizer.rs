
use image::{DynamicImage, ImageBuffer};
use image::ImageReader;
use image::imageops::FilterType;
use image::Rgb;

const BASE_WIDTH: u32 = 80;

pub fn resize(
  image: ImageBuffer<Rgb<u16>, Vec<u16>>,
) -> DynamicImage {
  let new_height = compute_new_height(&image);

  DynamicImage::resize_exact(
    &DynamicImage::ImageRgb16(image),
    BASE_WIDTH,
    new_height,
    FilterType::Lanczos3,
  )
}

fn compute_new_height(
  image: &ImageBuffer<Rgb<u16>, Vec<u16>>,
) -> u32 {
  let (w, h) = (image.width() as f32,
                          image.height() as f32);
  assert!(w != 0. && h != 0.);

  let ar = w/h;
  ((BASE_WIDTH as f32 / ar) / 2.) as u32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn correct_downscaled_dimensions() {
    let path = "rust-logo.png";
    let image = 
    ImageReader::open(path).unwrap()
      .decode().unwrap()
      .to_rgb16();
    let DynamicImage::ImageRgb16(image) = 
      resize(image) else {
        panic!("Expecting an Rgb16 image");
    };

    assert_eq!(image.dimensions(), (BASE_WIDTH, 40));
  }
}