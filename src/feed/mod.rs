mod parse;
mod strategy;

use crate::error::Error;

/// Podcast feed
#[derive(Debug)]
pub struct Podcast {
    /// Title of podcast
    pub title: String,
    /// Podcast episodes
    pub episodes: Vec<Episode>,
    /// Description of podcast
    pub description: Option<String>,
}

/// Podcast episode
#[derive(Debug)]
pub struct Episode {
    pub title: String,
    pub link: String,
    pub pub_date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub index: Option<usize>,
    pub author: Option<String>,
}

impl Episode {

    /// Return publication day
    pub fn publication_day(&self) -> u32 {
        0
    }

    /// Return publication month (1 indexed)
    pub fn publication_month(&self) -> u32 {
        0
    }

    /// Return publication year
    pub fn publication_year(&self) -> u32 {
        0
    }

}


pub async fn download_feed(url: &str) -> Result<Podcast, Error> {
    let strategy = strategy::create_strategy(url)?;
    strategy::execute_strategy(&strategy).await
}
