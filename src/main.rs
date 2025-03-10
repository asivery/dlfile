use std::{fs::File, io::copy, io::stdout};

use reqwest::blocking::Client;
use url::Url;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <url> [name]", args[0]);
        return;
    }
    let url = &args[1];
    let name = if args.len() > 2 {
        args[2].clone()
    } else {
        Url::parse(url)
            .unwrap()
            .path_segments()
            .unwrap()
            .last()
            .unwrap()
            .to_string()
    };
    eprintln!("Downloading '{}' to '{}'...", url, name);
    let mut response = Client::new().get(url).send().expect("Failed to fetch URL");
    if response.status().is_success() {
        if name == "-" {
            copy(&mut response, &mut stdout().lock()).unwrap();
        } else {
            let mut file = File::create(name).unwrap();
            copy(&mut response, &mut file).unwrap();
        }
    } else {
        eprintln!("Failed to download: HTTP {}", response.status());
    }
}
