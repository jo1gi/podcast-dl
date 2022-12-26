mod create;
mod execute;

use url::Url;

pub use create::create_strategy;
pub use execute::execute_strategy;

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

    fn try_op(mut self, op: UrlOperation) -> Self {
        self.operations.push(
            Operation::Try(op)
        );
        self
    }


}

enum Operation {
    Podcast(PodcastOperation),
    Url(UrlOperation),
    /// Skip if inner operation fails. Apply otherwise
    Try(UrlOperation),
}

/// Operation that returns a podcast object
enum PodcastOperation {
    RSS,
}

/// Operation that returns an url
#[derive(Debug)]
enum UrlOperation {
    /// Select url in json
    Json(Vec<&'static str>),
    /// Find rss link in html
    RssLink,
}
