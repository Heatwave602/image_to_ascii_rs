use clap::Parser;
use std::error::Error;

mod resizer;
use resizer::downscale;

use image::DynamicImage;

fn main() {
    let config = Config::parse();

    if let Err(e) = run(config) {
        println!("Error: {e}");
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let ds_image = downscale(&config.path)?;
    DynamicImage::save(&ds_image, change_file_name(&config.path))?;
    
    Ok(())
}

fn change_file_name(file_name: &str) -> String {
    file_name.replacen(".", "_ds.", 1)
}

#[derive(Parser)]
struct Config {
    path: String,
}
