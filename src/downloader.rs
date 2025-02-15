use std::path::PathBuf;

use tokio::fs::File;
use tokio::io::{BufWriter, AsyncWriteExt};
use futures_util::{StreamExt};
use indicatif::ProgressBar;
use reqwest::header::ACCEPT;

struct FileInfo {
    final_path: PathBuf,
}

impl FileInfo {
    fn new(url: &String, path: &PathBuf) -> Self {
        let file_name = url.split('/').next_back().unwrap();
        let final_path = path.join(file_name);
        return FileInfo {
            final_path,
        };
    }
}

pub struct Downloader<'a> {
    progress_bar: &'a ProgressBar,
}

impl<'a> Downloader<'a> {
    pub fn with_progress_bar(progress_bar: &'a ProgressBar) -> Self {
        Self { progress_bar }
    }
    pub async fn download(
        &self,
        url: &String,
        path: &std::path::PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header(ACCEPT, "application/pdf")
            .send()
            .await?;

        let url_info = FileInfo::new(url, path);

        let mut stream = response.bytes_stream();

        let file = File::create(format!("{}", url_info.final_path.display())).await?;
        let mut writer = BufWriter::new(file);

        let mut downloaded_length: u64 = 0;
        while let Some(chunk) = stream.next().await {
            let chunk_data = chunk.expect("chunk error");
            downloaded_length = downloaded_length + (chunk_data.len() as u64);
            writer.write(&chunk_data).await?;
        }

        self.progress_bar.inc(1);
        Ok(())
    }
}
