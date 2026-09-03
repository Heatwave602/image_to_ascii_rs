use clap::Parser;
use std::error::Error;

mod resizer;
mod ascii_converter;

use resizer::downscale;
use ascii_converter::map_to_ascii;

use image::{DynamicImage, GenericImageView};

fn main() {
    let config = Config::parse();

    if let Err(e) = run(config) {
        println!("Error: {e}");
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //image downscale
    let ds_image = downscale(&config.path)?;
    DynamicImage::save(&ds_image, change_file_name(&config.path))?;
    
    let gs_image = ds_image.to_luma8();
    assert!(gs_image.as_raw().len() == 3200);
    for (i, px) in gs_image.pixels().enumerate() {
        println!("Pixel {i}: {px:?}");
    }
    
    let it = ds_image.pixels().next();
    for ascii in map_to_ascii(ds_image.pixels()){
        println!("{ascii}");
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
