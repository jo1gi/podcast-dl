use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short, long, default_value="info", global = true)]
    pub log_level: log::LevelFilter,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// Download podcast episodes
    Download(Download),
    /// Print podcast info
    Print(Print),
    /// Search for a podcast
    #[cfg(feature = "search")]
    Search(Search),
}


#[derive(StructOpt)]
pub struct Download {
    /// Url of podcast to download
    pub url: String,
    /// Amount of episodes to download
    #[structopt(long)]
    pub limit: Option<usize>,
    /// Offset in episodes to download
    #[structopt(long)]
    pub offset: Option<usize>,
    /// Sort downloads by oldest instead of newest
    #[structopt(long)]
    pub oldest: bool,
    /// Output template
    #[structopt(short, long, default_value = "{podcast_title}/{episode_title}.mp3")]
    pub output: String,
    /// Download cover image
    #[structopt(long)]
    pub download_image: bool,
    /// Write description to disk
    #[structopt(long)]
    pub write_description: bool,
    /// Write episode description to file
    #[structopt(long)]
    pub write_episode_description: bool,
    /// Remove a string from the output
    #[structopt(long)]
    pub remove_from_output: Vec<String>,
}

#[derive(StructOpt)]
pub struct Print {
    /// Url of podcast to download
    pub url: String,
}

#[derive(StructOpt)]
pub struct Search {
    /// Search terms
    pub search_terms: Vec<String>,
}
