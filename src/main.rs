use std::error::Error;

use image_to_ascii::convert_to_ascii;

const COLOR_FLAG: &str = "-color";
const NOT_ENOUGH_ARGS_ERR: &str = 
r#"Not enough arguments

Usage:
cargo run -- <IMAGE_PATH> [COLOR_ASCII]"#;

fn main() {
    let config = Config::build(std::env::args())
        .unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            std::process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Error: {e}");
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
   println!("Configuration: {config:?}");

    // for c in ascii {
    //     print!("{c}");
    // }
    Ok(())
}

#[derive(Debug)]
struct Config {
    path:        String,
    color_ascii: bool
}

impl Config {
    fn build(
        mut args: std::env::Args
    ) -> Result<Config, &'static str> {
        args.next();
        let Some(path) = args.next() else {
            return Err(NOT_ENOUGH_ARGS_ERR);
        };
        let color_ascii = match args.next() {
            Some(flag) => flag == COLOR_FLAG,
            None => false,
        };

        Ok( Config { path, color_ascii } )
    }
}
