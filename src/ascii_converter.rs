use image::DynamicImage;
use image::Luma;

type LumaU8 = Luma<u8>;

const ASCII: &str = " .-=+*x#$&X@";

pub fn map_to_ascii(image: &DynamicImage) -> Vec<char> {
  let gs = image.to_luma8();
  let w = image.width();
  let pxs = gs.pixels();

  let (_, ascii) = pxs.fold(
    (0, vec![]),
    | (count, mut vec),
         px| {
          vec.push(to_ascii_luma(px));
          if count == w - 1 {vec.push('\n')}

          let count = (count + 1) % w;
          (count, vec)
    }
  );
  ascii
}

fn to_ascii_luma(px: &LumaU8) -> char {
  let val = px.0[0] as u16;
  let i = val * ASCII.len() as u16 / 256;
  ASCII.chars().nth(i.into()).
    expect("Error in ascii_converter::to_ascii_luma: mapping an ascii character gone wrong")
}