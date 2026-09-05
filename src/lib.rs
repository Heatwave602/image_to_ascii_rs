use image::ImageReader;

mod resizer;
use resizer::resize;

mod ascii_converter;
use ascii_converter::map_to_ascii;

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn convert_to_ascii(image_path: &str) -> Result<impl Iterator<Item = char>> {
  let image = 
  ImageReader::open(image_path)
    .unwrap()
    .decode()?;

  let ds_image = resize(&image);

  Ok(map_to_ascii(&ds_image).into_iter())
}