mod cli;
mod downloader;

use futures_util::future;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{self, BufRead};
use structopt::StructOpt;

async fn process_urls(urls: &Vec<String>, path: &std::path::PathBuf) {
    let progress_bar = ProgressBar::new(urls.len() as u64);
    progress_bar.set_message("Downloading files progress");

    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({per_sec}, {eta})")
        .progress_chars("#>-"));

    let mut futures = Vec::new();
    for url in urls {
        futures.push(downloader::download(&url, &path, &progress_bar));
    }

    let joined_futures = future::join_all(futures);
    joined_futures.await;

    progress_bar.finish_with_message(format!("Files are saved. 📦",));
}

fn read_urls_from_file(urls_file: &std::path::PathBuf) -> Result<Vec<String>, io::Error> {
    let file = File::open(urls_file).expect("File not found");
    let mut res = Vec::new();
    for line in io::BufReader::new(file).lines() {
        res.push(line?);
    }
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::from_args();
    println!("Arguments parsed: \n {}", args);

    let urls: Vec<String> = read_urls_from_file(&args.urls_file_path)?;

    process_urls(&urls, &args.result_dir_path).await;

    Ok(())
}
