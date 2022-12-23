mod create;

use url::Url;
use crate::{error::{Error, ParseError}, Podcast};
use super::parse::parse_rss_feed;

pub use create::create_strategy;

pub struct Strategy {
    url: Url,
    operations: Vec<Operation>,
}

impl Strategy {
    fn from_url(url: Url) -> Self {
        Self {
            url,
            operations: vec![],
        }
    }

    fn from_str(url: &str) -> Self {
        Self {
            url: Url::parse(url).unwrap(),
            operations: Vec::new(),
        }
    }

    fn add_url_operation(mut self, op: UrlOperation) -> Self {
        self.operations.push(
            Operation::Url(op)
        );
        self
    }

    fn add_podcast_operation(mut self, op: PodcastOperation) -> Self {
        self.operations.push(
            Operation::Podcast(op)
        );
        self
    }

    fn rss(self) -> Self {
        self.add_podcast_operation(PodcastOperation::RSS)
    }

    fn json(self, lookup: Vec<&'static str>) -> Self {
        self.add_url_operation(UrlOperation::Json(lookup))
    }


}

enum Operation {
    Podcast(PodcastOperation),
    Url(UrlOperation),
}

/// Operation that returns a podcast object
enum PodcastOperation {
    RSS,
}

/// Operation that returns an url
enum UrlOperation {
    Json(Vec<&'static str>)
}

pub async fn execute_strategy(strategy: &Strategy) -> Result<Podcast, Error> {
    let mut url = strategy.url.clone();
    for operation in &strategy.operations {
        let content = reqwest::get(url)
            .await?
            .bytes()
            .await?;
        match operation {
            Operation::Podcast(op) => return Ok(execute_podcast_operation(&op, &content)?),
            Operation::Url(op) => {
                url = execute_url_operation(&op, &content)?;
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
    let url = match op {
        UrlOperation::Json(lookup) => {
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
            json.as_str().unwrap().to_string()
        }
    };
    Ok(Url::parse(&url)?)
}
