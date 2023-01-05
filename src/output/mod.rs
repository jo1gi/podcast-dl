/// Iterator over episode
mod episode_iterator;
/// Module for formatting episode as strings
mod formatting;
/// Struct for storing write options
mod write_options;

use crate::{
    error::Error,
    logging,
    Podcast, Episode,
};
use std::{
    path::{PathBuf, Path},
    str::FromStr,
    io::Write,
};
use reqwest::Client;
use episode_iterator::EpisodeIterator;
pub use write_options::WriteOptions;

struct WriteData<'a> {
    podcast: &'a Podcast,
    client: Client,
    options: &'a WriteOptions,
}

/// Downloads all episodes in `podcast` based on `options`
pub async fn download_podcast(podcast: &Podcast, options: &WriteOptions) -> Result<(), Error> {
    let write_data = WriteData {
        podcast, options,
        client: Client::new()
    };
    let episodes = EpisodeIterator::new(podcast, options);
    for episode in episodes {
        let result = download_episode(episode, &write_data).await;
        logging::print_download_status(&result);
    }
    Ok(())
}

/// Writes a single episode to disk
async fn download_episode<'a>(episode: &Episode, write_data: &WriteData<'a>) -> Result<(), Error> {
    let audio_path = create_path(episode, write_data)?;
    log::info!("Downloading {}", episode.title);
    download_episode_audio(episode, &audio_path, &write_data.client).await?;
    if write_data.options.write_episode_description {
        write_episode_description(episode, &audio_path)?;
    }
    Ok(())
}

/// Download episode audio and write it to file
async fn download_episode_audio(episode: &Episode, path: &PathBuf, client: &Client) -> Result<(), Error> {
    let data = client.get(&episode.link)
        .send().await?
        .bytes().await?;
    let mut file = std::fs::File::create(&path)?;
    file.write_all(&data)
        .or(Err(Error::WriteToFile("audio")))?;
    Ok(())
}

/// Write episode description to file
fn write_episode_description(episode: &Episode, audio_path: &Path) -> Result<(), Error> {
    if let Some(description) = &episode.description {
        let description_path = audio_path.with_extension("description.txt");
        let mut file = std::fs::File::create(&description_path)?;
        file.write_all(description.as_bytes())
            .or(Err(Error::WriteToFile("description")))?;
        Ok(())
    } else {
        Err(Error::ValueMissing{ value: "description", from: "episode" })
    }
}

/// Creates a pathbuf from string and creates its parent directory if it does not exist
fn create_path(episode: &Episode, write_data: &WriteData) -> Result<PathBuf, Error> {
    // Create path
    let formatted = formatting::format_episode(write_data.podcast, episode, &write_data.options)?;
    let pathbuf = PathBuf::from_str(&formatted).unwrap();
    // Check if file exists
    if pathbuf.exists() {
        return Err(Error::FileExists(episode.title.clone()));
    }
    // Check if directory exists
    if !pathbuf.parent().unwrap().exists() {
        std::fs::create_dir_all(pathbuf.parent().unwrap())?;
    }
    return Ok(pathbuf);
}

/// Creates a path to a file in the same directory as the first episode
fn create_non_episode_path(podcast: &Podcast, file_name: &str, options: &WriteOptions) -> Result<PathBuf, Error> {
    let formatted = formatting::format_episode(podcast, &podcast.episodes[0], options)?;
    PathBuf::from_str(&formatted).ok()
        .and_then(|path| Some(PathBuf::from(path.parent()?)))
        .map(|parent| parent.join(file_name))
        .ok_or(Error::Path)
}

/// Downloads cover image of podcast in the same folder as the first episode as cover.jpg
pub async fn download_image(podcast: &Podcast, options: &WriteOptions) -> Result<(), Error> {
    if let Some(url) = &podcast.image {
        // Download
        let image = reqwest::get(url).await?
            .bytes().await?;
        // Write
        let cover_path = create_non_episode_path(podcast, "cover.jpg", options)?;
        let mut file = std::fs::File::create(&cover_path)?;
        file.write_all(&image)?;
        Ok(())
    // Image missing from podcast
    } else {
        Err(Error::ValueMissing{ value: "image", from: "podcast" })
    }
}

/// Writes podcast description to disk in the same folder as the first episode
pub async fn write_description(podcast: &Podcast, options: &WriteOptions) -> Result<(), Error> {
    if let Some(description) = &podcast.description {
        let description_path = create_non_episode_path(podcast, "description.txt", options)?;
        let mut file = std::fs::File::create(&description_path)?;
        file.write_all(description.as_bytes())?;
        Ok(())
    } else {
        Err(Error::ValueMissing{ value: "description", from: "podcast" })
    }
}
