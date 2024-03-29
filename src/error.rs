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
    /// Format string
    StringFormat,
    /// Failed to create path
    Path,
    /// File aldready exists
    FileExists(String),
    /// Could not write {0} file
    WriteToFile(&'static str),
    /// Missing {value} from {from}
    ValueMissing {
        value: &'static str,
        from: &'static str,
    },
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
    /// Missing element
    MissingElement,
}
