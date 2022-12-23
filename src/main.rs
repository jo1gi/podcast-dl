mod args;
mod error;
mod feed;
mod logging;
mod output;

pub use error::Error;
use structopt::StructOpt;
pub use feed::{Podcast, Episode};
use output::WriteOptions;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let args = args::Args::from_args();
    logging::setup_logger(args.log_level)?;
    let podcast = feed::download_feed(&args.url).await?;
    logging::print_podcast(&podcast);
    output::download_episodes(&podcast, &WriteOptions::default()).await?;
    Ok(())
}


