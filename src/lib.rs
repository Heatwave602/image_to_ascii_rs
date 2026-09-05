use image::{DynamicImage, ImageBuffer};

mod resizer;
mod ascii_converter;

use resizer::resize;
use ascii_converter::convert_to_ascii;

type ImageBuf = ImageBuffer<image::Rgb<u16>, Vec<u16>>;

pub fn resize_image(image: ImageBuf) -> DynamicImage {
  resize(image)
}

pub fn map_to_ascii(image: &DynamicImage) -> impl Iterator<Item = char> {
  convert_to_ascii(image)
}