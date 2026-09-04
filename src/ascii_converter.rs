use image::DynamicImage;
use image::Luma;

type LumaU8 = Luma<u8>;

pub fn map_to_ascii(image: &DynamicImage) -> impl Iterator<Item = char> {
  let gs = image.to_luma8();
  let w = image.width();
  let pxs = gs.pixels();

  let (_, ascii) = pxs.fold(
    (0, vec![]),
    | (count, mut vec),
         px| {
          vec.push(to_ascii(&px));
          if count == w - 1 {vec.push('\n')}

          let count = (count + 1) % w;
          (count, vec)
    }
  );

  ascii.into_iter()

  // let ascii = pxs.fold(
  //   (0, Vec::<char>::new()),
  //   | mut acc, px | {
  //     acc(1).push(into_ascii(&px));
  //     if eol(&px, image.width()) {
  //       acc.push('\n');
  //     }
  //     acc
  //   }
  // );

  // ascii.into_iter()
}

fn to_ascii(_px: &LumaU8) -> char {
  '*'
}