use log::{Level, LevelFilter, Metadata};
use colored::{Color, Colorize};
use crate::{Podcast, search::SearchResult};

/// Setup logging system
pub fn setup_logger(level: LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let (first, rest, color) = format_log_message(
                message.to_string(),
                record.level(),
                record.target()
            );
            out.finish(format_args!(
                "{:>12} {}",
                first.bold().color(color),
                rest,
            ))
        })
        .level(level)
        .filter(|metadata| {
            (metadata.level() != Level::Debug && metadata.level() != Level::Trace)
            || filter_log_message(metadata)
        })
        .chain(std::io::stderr())
        .apply()?;
    Ok(())
}

/// Filter out log messages based on target
fn filter_log_message(metadata: &Metadata) -> bool {
    ![
        "selectors::matching",
        "html5ever::tokenizer",
        "html5ever::tokenizer::char_ref",
        "html5ever::tree_builder",
    ].contains(&metadata.target())
}

fn format_log_message(msg: String, level: Level, target: &str) -> (String, String, Color) {
    match level {
        Level::Error => ("ERROR".to_string(), msg, Color::Red),
        Level::Warn => ("WARNING".to_string(), msg, Color::Yellow),
        Level::Debug => ("DEBUG".to_string(), format!("{} {}", msg, target.bright_black()), Color::Yellow),
        Level::Trace => ("TRACE".to_string(), format!("{} {}", msg, target.bright_black()), Color::Cyan),
        _ => {
            let split = msg.find(" ").unwrap();
            let first_word = msg[..split].to_string();
            let rest = msg[split+1..].to_string();
            let color = match first_word.as_str() {
                "Downloading" | "Skipping" => Color::Blue,
                _ => Color::BrightYellow,
            };
            (first_word, rest, color)
        }
    }
}

pub fn print_podcast(podcast: &Podcast) {
    println!("{:#?}", podcast);
}

#[cfg(feature = "search")]
pub fn print_search_results(search_results: &Vec<SearchResult>) {
    println!(
        "{:<35} {:<35} {}",
        "Title".bold().green(),
        "Artist".bold().cyan(),
        "Feed".bold().white()
    );
    for result in search_results {
        println!(
            "{:<35} {:<35} {}",
            result.title.green(),
            result.artist.cyan(),
            result.url.white(),
        );
    }
}
