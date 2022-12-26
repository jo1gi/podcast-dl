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
}

#[derive(StructOpt)]
pub struct Print {
    /// Url of podcast to download
    pub url: String,
}
