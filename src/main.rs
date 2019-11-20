#[macro_use]
extern crate clap;

use std::fs;

fn main() {
    let matches = clap::App::new("wsync")
        .version("0.1.0")
        .about("web sync utility")
        .arg(
            clap::Arg::with_name("INPUT")
                .help("URL to download from.")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("include")
                .short("i")
                .long("include")
                .help("Files to include in the download.")
                .multiple(true)
                .conflicts_with("exclude")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("exclude")
                .short("e")
                .long("exclude")
                .help("Files to exclude from the download.")
                .multiple(true)
                .conflicts_with("include")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("out")
                .short("o")
                .long("out")
                .help("Target download directory.")
                .takes_value(true)
                .default_value("."),
        )
        .get_matches();
}
