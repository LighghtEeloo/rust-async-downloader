use futures_util::future;
use indicatif::{ProgressBar, ProgressStyle};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use structopt::StructOpt;

use crate::downloader;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub urls_file_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    pub result_dir_path: std::path::PathBuf,
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(({})) >>= {}",
            self.urls_file_path.display(),
            self.result_dir_path.display()
        )
    }
}

impl Cli {
    pub fn to_target_urls(&self) -> Result<Vec<String>, io::Error> {
        let file = File::open(self.urls_file_path.to_owned()).expect("File not found");
        let mut res = Vec::new();
        for line in io::BufReader::new(file).lines() {
            res.push(line?);
        }
        Ok(res)
    }
    pub async fn process_urls(&self, urls: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let path = &self.result_dir_path;
        let progress_bar = ProgressBar::new(urls.len() as u64);
        progress_bar.set_message("Downloading files progress");

        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({per_sec}, {eta})")
            .progress_chars("#>-"));

        let downloader = downloader::Downloader::with_progress_bar(&progress_bar);

        let mut futures = Vec::new();
        for url in urls {
            futures.push(downloader.download(&url, &path));
        }

        let joined_futures = future::join_all(futures);
        joined_futures.await;

        progress_bar.finish_with_message(format!("Files are saved. 📦",));
        Ok(())
    }
}
