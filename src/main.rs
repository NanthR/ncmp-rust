mod config;
mod connections;
mod draw;
mod drawing_utils;
mod events;
mod utils;

use draw as Draw;

use clap::{App, Arg};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("ncmp-rust")
        .version(VERSION)
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .takes_value(true)
                .help("Connect to server at host")
                .default_value(config::PORT),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Connect to server at port")
                .default_value(config::HOST),
        )
        .get_matches();

    Draw::draw()?;
    Ok(())
}
