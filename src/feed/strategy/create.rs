use crate::error::ParseError;
use super::Strategy;
use url::Url;

pub fn create_strategy(input: &str) -> Result<Strategy, ParseError> {
    let url = Url::parse(input)?;
    let host = url.host_str();
    if host == Some("rss.com") {
        return Ok(rss_dot_com(&url));
    }
    if host == Some("podcasts.apple.com") {
        return Ok(itunes(&url));
    }
    Ok(Strategy::from_url(url)
        .rss())
}

fn last_path_part(url: &Url) -> Option<&str> {
    url.path_segments()
        .unwrap()
        .filter(|x| !x.is_empty())
        .last()
}

fn itunes(url: &Url) -> Strategy {
    let itunes_id = last_path_part(url).unwrap()[2..].to_string();
    Strategy::from_str(&format!("https://itunes.apple.com/lookup?id={}", itunes_id))
        .json(vec!["results", "feedUrl"])
        .rss()
}

fn rss_dot_com(url: &Url) -> Strategy {
    let id = last_path_part(url).unwrap();
    Strategy::from_str(&format!("https://media.rss.com/{}/feed.xml", id))
        .rss()
}
