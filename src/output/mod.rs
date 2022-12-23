mod episode_iterator;
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

pub async fn download_episodes(podcast: &Podcast, options: &WriteOptions) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let episodes = EpisodeIterator::new(podcast, options);
    let output_dir = get_dir(podcast)?;
    for episode in episodes {
        write_episode(episode, &output_dir, &client).await?;
    }
    Ok(())
}

async fn write_episode(episode: &Episode, output_dir: &PathBuf, client: &reqwest::Client) -> Result<(), Error> {
    let file_path = output_dir.join(&format!("{}.mp3", episode.title));
    if file_path.exists() {
        log::info!("Skipping {}", episode.title);
        return Ok(());
    }
    log::info!("Downloading {}", episode.title);
    let mut file = std::fs::File::create(&file_path)?;
    let data = client.get(&episode.link)
        .send()
        .await?
        .bytes()
    .await?;
    match file.write_all(&data) {
        Ok(()) => (),
        Err(e) => log::warn!("Failed to write file {}", e),
    }
    Ok(())

}

fn get_dir(podcast: &Podcast) -> Result<PathBuf, Error> {
    let dir = PathBuf::from_str(&podcast.title).unwrap();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    return Ok(dir);
}
