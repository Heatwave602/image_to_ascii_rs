use clap::Parser;
use std::error::Error;

use image::{DynamicImage, ImageReader};

use image_to_ascii::{resize_image, map_to_ascii};

fn main() {
    let config = Config::parse();

    if let Err(e) = run(config) {
        println!("Error: {e}");
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = &config.path;
    let image = ImageReader::open(path).unwrap()
      .decode().unwrap()
      .to_rgb16();

    let ds_image = resize_image(image);
    DynamicImage::save(&ds_image, change_file_name(&config.path))?;
    
    for c in map_to_ascii(&ds_image){
        print!("{c}");
    };
    
    Ok(())
}

fn change_file_name(file_name: &str) -> String {
    file_name.replacen(".", "_ds.", 1)
}

#[derive(Parser)]
struct Config {
    path: String,
}
