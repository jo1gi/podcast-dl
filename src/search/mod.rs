mod search_engines;

use crate::error::Error;
use reqwest::Client;
use search_engines::*;

pub struct SearchResult {
    /// Title of podcast
    pub title: String,
    /// Url of podcast
    pub url: String,
    /// Who made the podacst
    pub artist: String,
    /// Url of podcast cover image
    pub artwork: String,
    /// The search engine the result was found on
    pub search_engine: SearchEngine,
}

type SearchArgs = Vec<String>;

#[derive(Copy, Clone)]
pub enum SearchEngine {
    Itunes,
    PodcastIndex,
}

macro_rules! run_search_engines {
    ($search_terms:expr, $($search_engine:expr),+) => {{
        let client = Client::new();
        let mut output = Vec::new();
        $({
            let mut results = match $search_engine($search_terms, &client).await {
                Ok(results) => results,
                Err(error) => {
                    log::error!("{}", error);
                    Vec::new()
                }
            };
            output.append(&mut results);
        })+
        output.dedup_by_key(|x| x.url.clone());
        return Ok(output);
    }}
}

/// Search for podcast feeds
pub async fn search(search_terms: &Vec<String>) -> Result<Vec<SearchResult>, Error> {
    run_search_engines!(
        search_terms,
        itunes_search,
        podcast_index_search
    )
}
