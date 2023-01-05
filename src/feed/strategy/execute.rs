use crate::{
    error::{Error, ParseError},
    Podcast,
    feed::{
        strategy::{Strategy, Operation, PodcastOperation, UrlOperation},
        parse::parse_rss_feed,
    },
};
use url::Url;

pub async fn execute_strategy(strategy: &Strategy) -> Result<Podcast, Error> {
    let mut url = strategy.url.clone();
    for operation in &strategy.operations {
        let content = reqwest::get(url.clone())
            .await?
            .bytes()
            .await?;
        match operation {
            Operation::Podcast(op) => return Ok(execute_podcast_operation(&op, &content)?),
            Operation::Url(op) => {
                url = execute_url_operation(&op, &content)?;
            },
            Operation::Try(op) => {
                log::debug!("Using try operation: {:?}", op);
                match execute_url_operation(&op, &content) {
                    Ok(x) => url = x,
                    Err(_) => log::debug!("Try operation did not succeed"),
                }
            }
        }
    }
    unreachable!()
}

fn execute_podcast_operation(op: &PodcastOperation, content: &bytes::Bytes) -> Result<Podcast, ParseError> {
    match op {
        PodcastOperation::RSS => parse_rss_feed(content)
    }
}

fn execute_url_operation(op: &UrlOperation, content: &bytes::Bytes) -> Result<Url, ParseError> {
    log::debug!("Executing url operation: {:?}", op);
    let url = match op {
        UrlOperation::Json(lookup) => execute_url_json(content, lookup)?,
        UrlOperation::RssLink => execute_url_rsslink(content)?,
    };
    Ok(Url::parse(&url)?)
}

fn execute_url_json(content: &bytes::Bytes, lookup: &Vec<&str>) -> Result<String, ParseError> {
    let text = std::str::from_utf8(content)?;
    let mut json: serde_json::Value = serde_json::from_str(text)?;
    let mut iter = lookup.iter();
    loop {
        if json.is_array() {
            json = json[0].take();
        } else if json.is_object() {
            json = json[iter.next().unwrap()].take();
        } else {
            break;
        }
    }
    Ok(json.as_str().unwrap().to_string())
}

fn execute_url_rsslink(content: &bytes::Bytes) -> Result<String, ParseError> {
    let text = std::str::from_utf8(content)?;
    let doc = scraper::Html::parse_document(&text);
    let selector = scraper::Selector::parse(r#"link[type="application/rss+xml"]"#)
        .unwrap();
    let result = doc.select(&selector)
        .find_map(|link| link.value().attr("href").map(String::from))
        .ok_or(ParseError::MissingElement)?;
    Ok(result)
}
