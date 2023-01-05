use log::{Level, LevelFilter, Metadata};
use colored::{Color, Colorize};
use crate::{Podcast, search::SearchResult, Error};

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

fn text_wrap(text: &str, length: u32) -> String {
    let mut output = String::new();
    let mut current_line_length = 0;
    for part in text.split(" ") {
        if current_line_length > length {
            output.push('\n');
            current_line_length = 0;
        }
        output.push_str(&part);
        output.push(' ');
        current_line_length += part.len() as u32;
    }
    return output;
}

fn print_title(title: &str, value: &str) {
    println!("{}\n{}\n", title.cyan().bold(), value);
}

pub fn print_podcast(podcast: &Podcast) {
    print_title("Title", &podcast.title);
    if let Some(description) = &podcast.description {
        print_title("Description", &text_wrap(&description, 40));
    }
    println!("{}", "Episodes".cyan().bold());
    for episode in &podcast.episodes {
        println!("- {}", episode.title);
    }
}

pub fn shorten_to_length(input: &str, len: usize) -> String {
    if input.len() > len {
       input[0..len].to_string() + "…"
    } else {
        input.to_string()
    }
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
        let formatted_title = &shorten_to_length(&result.title, 34);
        let formatted_artist = &shorten_to_length(&result.artist, 34);
        println!(
            "{:<35} {:<35} {}",
            formatted_title.green(),
            formatted_artist.cyan(),
            result.url.white(),
        );
    }
}

pub fn print_download_status(result: &Result<(), Error>) {
    if let Err(error) = result {
        match error {
            Error::FileExists(title) => log::info!("Skipping {}", title),
            e => log::error!("{}", e),
        }
    }
}
