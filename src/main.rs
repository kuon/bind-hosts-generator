use reqwest::Url;
use serde::Deserialize;
use std::io::Write;
use std::fs;
use std::fs::File;
use structopt::StructOpt;

#[derive(Deserialize)]
struct Config {
    host_file_url: String,
    output_file: std::path::PathBuf,
    header: String,
    ip: String,
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(
        parse(from_os_str),
        short = "c",
        long = "config",
        default_value = "config.toml"
    )]
    config: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let config = fs::read(args.config).unwrap();
    let config = String::from_utf8_lossy(&config);
    let config: Config = toml::from_str(&config).unwrap();

    let host_data = reqwest::get(Url::parse(&config.host_file_url).unwrap())
        .unwrap()
        .text()
        .unwrap();

    let mut output_file = File::create(config.output_file).unwrap();

    write!(output_file, "{}\n", config.header).unwrap();

    for line in host_data.lines() {
        if line.starts_with("0.0.0.0 ") {
            let range =
            if let Some(idx) = line.find('#') {
                8..idx
            } else {
                8..line.len()
            };
            if let Some(host) = line.get(range) {
                write!(output_file, "{} A {}\n", host, config.ip).unwrap();
            }
        }
    }
    output_file.sync_all().unwrap();
}

