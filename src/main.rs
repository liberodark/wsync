use std::fs;
use std::path;

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

    let out_dir = path::Path::new(matches.value_of("out").unwrap());
    let client = reqwest::Client::new();
    let mut res = client
        // https://shibboleth-mirror.cdi.ti.ja.net/CentOS_7/
        .get(matches.value_of("INPUT").unwrap())
        .send()
        .unwrap();

    let content_type = res.headers().get("Content-Type").unwrap().to_str().unwrap();
    let is_dir = regex::Regex::new("text/html")
        .unwrap()
        .is_match(content_type);
    let body = res.text().unwrap();
    if !out_dir.exists() {
        fs::create_dir_all(out_dir).unwrap();
    }

    fs::write(out_dir.join("index.html"), body).unwrap();
}
