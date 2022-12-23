use log::{Level, LevelFilter};
use colored::{Color, Colorize};
use crate::Podcast;

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
        .chain(std::io::stderr())
        .apply()?;
    Ok(())
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
