use clap::Parser;
use std::error::Error;

use image_to_ascii::convert_to_ascii;

fn main() {
    let config = Config::parse();

    if let Err(e) = run(config) {
        println!("Error: {e}");
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for c in convert_to_ascii(&config.path)? {
        print!("{c}");
    }
    Ok(())
}

// fn change_file_name(file_name: &str) -> String {
//     file_name.replacen(".", "_ds.", 1)
// }

#[derive(Parser)]
struct Config {
    path: String,
}
