
use image::DynamicImage;
use image::imageops::FilterType;

const BASE_WIDTH: u32 = 80;

pub fn resize(
  image: &DynamicImage,
) -> DynamicImage {
  DynamicImage::resize_exact(
    image,
    BASE_WIDTH,
    compute_new_height(image),
    FilterType::Lanczos3,
  )
}

fn compute_new_height(
  image: &DynamicImage,
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
    image::ImageReader::open(path).unwrap()
      .decode().unwrap();

    let image = resize(&image);
    assert_eq!((image.width(),  image.height()),
               (BASE_WIDTH,     40));
  }
}