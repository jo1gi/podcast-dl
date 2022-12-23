use thiserror::Error;
use displaydoc::Display;

#[derive(Debug, Error, Display)]
pub enum Error {
    /// Failed to download: $0
    Download(#[from] reqwest::Error),
    /// IO Error
    IO(#[from] std::io::Error),
    /// Logging
    InitLogging(#[from] fern::InitError),
    /// Parsing
    Parse(#[from] ParseError),
}

#[derive(Debug, Error, Display)]
pub enum ParseError {
    /// Failed to parse rss: $0
    RSS(#[from] rss::Error),
    /// Failed to parse input url: $0
    URL(#[from] url::ParseError),
    /// Parsing bytes as utf8
    UTF8(#[from] std::str::Utf8Error),
    /// Json
    Json(#[from] serde_json::Error),
}
