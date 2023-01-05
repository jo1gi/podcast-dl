use crate::{
    error::{Error, ParseError},
    search::{SearchResult, SearchEngine, SearchArgs},
};
use reqwest::Client;

/// Search for podcast feeds on Podcast Index
pub async fn podcast_index_search(search_terms: &SearchArgs, client: &Client) -> Result<Vec<SearchResult>, Error> {
    itunes_search_format(
        search_terms,
        client,
        "https://api.podcastindex.org/search?term=",
        SearchEngine::PodcastIndex,
    ).await
}


/// Search for podcast feeds on Itunes
pub async fn itunes_search(search_terms: &SearchArgs, client: &Client) -> Result<Vec<SearchResult>, Error> {
    itunes_search_format(
        search_terms,
        client,
        "https://itunes.apple.com/search?media=podcast&term=",
        SearchEngine::Itunes,
    ).await
}

/// Search for podcast feeds on sites using the same format as Itunes
async fn itunes_search_format(
    search_terms: &SearchArgs,
    client: &Client,
    base_url: &str,
    engine: SearchEngine
) -> Result<Vec<SearchResult>, Error> {
    // Format search url
    let formatted_terms = search_terms.join("+");
    let url = format!("{}{}", base_url, formatted_terms);
    // Create request
    let response: serde_json::Value = client.get(&url)
        .send().await?
        .json().await?;
    // Create search objects
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
                search_engine: engine,
            })
        })
        .collect();
    return Ok(results);
}
