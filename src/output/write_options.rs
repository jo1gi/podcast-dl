/// Options for downloading a podcast
#[derive(Default)]
pub struct WriteOptions {
    /// How many episodes should be downloaded
    pub limit: Option<usize>,
    /// Offset in episodes to download
    pub offset: Option<usize>,
    /// Start with the oldest episodes
    pub oldest: bool,
    /// Output template
    pub template: String,
}
