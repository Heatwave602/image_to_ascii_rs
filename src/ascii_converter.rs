use image::{DynamicImage, GenericImageView};

pub fn map_to_ascii(image: &DynamicImage) -> impl Iterator<Item = char> {
  let pxs = image.pixels();

  let ascii = pxs.fold(
    Vec::<char>::new(),
    |mut acc,  px| {
      acc.push(into_ascii(&px));
      if eol(&px, image.width()) {
        acc.push('\n');
      }
      acc
    }
  );

  ascii.into_iter()
}

fn into_ascii(px: impl Pixel) -> char {
  unimplemented!("map luminance value to char")
}

fn eol((x, _, _): Pixel, width: u32) -> bool {
  x == width - 1
}