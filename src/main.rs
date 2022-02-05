mod cli;
mod downloader;

use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::from_args();
    println!("{args}");
    let urls: Vec<String> = args.to_target_urls()?;
    args.process_urls(&urls).await?;
    Ok(())
}
