use crate::error::ParseError;
use super::Strategy;
use url::Url;

pub fn create_strategy(input: &str) -> Result<Strategy, ParseError> {
    let url = Url::parse(input)?;
    let host = url.host_str();
    let strategy = if host == Some("rss.com") {
        rss_dot_com(&url)
    } else if host == Some("podcasts.apple.com") {
        itunes(&url)
    } else {
        Strategy::from_url(url)
            .try_op(super::UrlOperation::RssLink)
            .rss()
    };
    Ok(strategy)
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
