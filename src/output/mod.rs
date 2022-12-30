mod episode_iterator;
mod formatting;
mod write_options;

use crate::{
    error::Error,
    Podcast, Episode,
};

use std::{
    path::PathBuf,
    str::FromStr,
    io::Write,
};

use episode_iterator::EpisodeIterator;
pub use write_options::WriteOptions;

/// Downloads all episodes in `podcast` based on `options`
pub async fn download_episodes(podcast: &Podcast, options: &WriteOptions) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let episodes = EpisodeIterator::new(podcast, options);
    for episode in episodes {
        let path = formatting::format_episode(podcast, episode, &options.template)?;
        write_episode(episode, &path, &client).await?;
    }
    Ok(())
}

/// Writes a single episode to disk
async fn write_episode(episode: &Episode, path: &str, client: &reqwest::Client) -> Result<(), Error> {
    // Creating path
    let file_path = create_path(path)?;
    if file_path.exists() {
        log::info!("Skipping {}", episode.title);
        return Ok(());
    }
    // Downloading episode
    log::info!("Downloading {}", episode.title);
    let mut file = std::fs::File::create(&file_path)?;
    let data = client.get(&episode.link)
        .send()
        .await?
        .bytes()
        .await?;
    // Writing file
    match file.write_all(&data) {
        Ok(()) => (),
        Err(e) => log::warn!("Failed to write file {}", e),
    }
    Ok(())

}

/// Creates a pathbuf from string and creates its parent directory if it does not exist
fn create_path(path: &str) -> Result<PathBuf, Error> {
    let pathbuf = PathBuf::from_str(path).unwrap();
    if !pathbuf.parent().unwrap().exists() {
        std::fs::create_dir_all(pathbuf.parent().unwrap())?;
    }
    return Ok(pathbuf);
}

/// Creates a path to a file in the same directory as the first episode
fn create_non_episode_path(podcast: &Podcast, file_name: &str, template: &str) -> Result<PathBuf, Error> {
    let formatted = formatting::format_episode(podcast, &podcast.episodes[0], template)?;
    PathBuf::from_str(&formatted)
        .ok()
        .and_then(|path| Some(PathBuf::from(path.parent()?)))
        .map(|path| path.join(file_name))
        .ok_or(Error::Path)
}

/// Downloads cover image of podcast in the same folder as the first episode as cover.jpg
pub async fn download_image(podcast: &Podcast, options: &WriteOptions) -> Result<(), Error> {
    let cover_path = create_non_episode_path(podcast, "cover.jpg", &options.template)?;
    if let Some(url) = &podcast.image {
        let image = reqwest::get(url)
            .await?
            .bytes()
            .await?;
        let mut file = std::fs::File::create(&cover_path)?;
        file.write_all(&image);
    } else {
        log::warn!("Could not download cover image. Image not found")
    }
    Ok(())
}
