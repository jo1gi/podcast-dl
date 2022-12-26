use crate::error::{Error, ParseError};

use reqwest::Client;

pub struct SearchResult {
    /// Title of podcast
    pub title: String,
    /// Url of podcast
    pub url: String,
    pub artist: String,
    pub artwork: String,
    pub search_engine: SearchEngine,
}

pub enum SearchEngine {
    Itunes
}

/// Search for podcast feeds
pub async fn search(search_terms: &str) -> Result<Vec<SearchResult>, Error> {
    let client = Client::new();
    itunes_search(search_terms, &client).await
}

/// Search for podcast feeds on Itunes
async fn itunes_search(search_terms: &str, client: &Client) -> Result<Vec<SearchResult>, Error> {
    let formatted_terms = search_terms.replace(" ", "+");
    let url = format!(
        "https://itunes.apple.com/search?media=podcast&term={}",
        formatted_terms
    );
    let response: serde_json::Value = client.get(&url)
        .send()
        .await?
        .json()
        .await?;
    let results = response["results"]
        .as_array()
        .ok_or(ParseError::MissingElement)?
        .iter()
        .filter_map(|result| {
            Some(SearchResult {
                title: result["collectionName"].as_str()?.to_string(),
                url: result["feedUrl"].as_str()?.to_string(),
                artist: result["artistName"].as_str()?.to_string(),
                artwork: result["artworkUrl600"].as_str()?.to_string(),
                search_engine: SearchEngine::Itunes,
            })
        })
        .collect();
    return Ok(results);
}
