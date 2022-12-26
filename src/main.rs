mod args;
mod error;
mod feed;
mod logging;
mod output;

pub use error::Error;
pub use feed::{Podcast, Episode};

use structopt::StructOpt;
use output::WriteOptions;
use args::Command;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let args = args::Args::from_args();
    logging::setup_logger(args.log_level)?;
    match args.command {
        Command::Download(download_args) => download(&download_args).await?,
        Command::Print(print_args) => print(&print_args).await?,
    }
    Ok(())
}

/// Download episodes
async fn download(args: &args::Download) -> Result<(), error::Error> {
    let podcast = feed::download_feed(&args.url).await?;
    let write_options = WriteOptions {
        limit: args.limit,
        offset: args.offset,
        oldest: args.oldest,
        template: args.output.clone(),
    };
    output::download_episodes(&podcast, &write_options).await?;
    Ok(())
}

/// Print podcast info
async fn print(args: &args::Print) -> Result<(), error::Error> {
    let podcast = feed::download_feed(&args.url).await?;
    logging::print_podcast(&podcast);
    Ok(())
}
