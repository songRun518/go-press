use std::collections::HashSet;
use std::error::Error;
use std::io::{Cursor, Write};
use std::path::Path;
use std::{env, fs, io};

use reqwest::blocking;
use scraper::{Html, Selector};

fn main() {
    let bytes = download().unwrap();
    let target_dir = find_env().unwrap_or_else(ask_target);
    let target_dir = Path::new(&target_dir);
    println!("Extracting zip...");
    zip_extract::extract(Cursor::new(bytes), target_dir, true).unwrap();

    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn download() -> Result<Vec<u8>, Box<dyn Error>> {
    println!("Getting the latest version...");
    let download_page = "https://go.dev/dl/";
    let resp = blocking::get(download_page)?;
    if !resp.status().is_success() {
        Err("Download Page Error")?;
    }

    let doc = Html::parse_document(&resp.text()?);
    let verison = doc
        .select(&Selector::parse("div.toggleVisible").unwrap())
        .next()
        .unwrap()
        .attr("id")
        .unwrap();

    let filename = format!("{verison}.windows-amd64.zip");
    println!("Downloading {filename}...");
    let url = format!("https://go.dev/dl/{filename}");
    let resp = blocking::ClientBuilder::default()
        .timeout(None)
        .build()?
        .get(url)
        .send()?;

    Ok(resp.bytes()?.to_vec())
}

fn find_env() -> Option<String> {
    let path = env::var("Path").unwrap_or_default();
    let paths = path.split(';').collect::<HashSet<_>>();
    let go_exist = paths
        .iter()
        .filter_map(|path| fs::read_dir(path).ok())
        .flatten()
        .filter_map(|dir| dir.ok())
        .find(|dir| dir.file_name() == "go.exe");

    go_exist.map(|go_dir| {
        go_dir
            .path()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_string_lossy()
            .to_string()
    })
}

fn ask_target() -> String {
    let mut target = String::new();
    print!("Where to place go: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut target).unwrap();

    let target = target.trim().to_string();
    if !fs::exists(&target).unwrap() {
        fs::create_dir_all(&target).unwrap();
    }

    target
}
