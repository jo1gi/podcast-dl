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

pub async fn download_episodes(podcast: &Podcast, options: &WriteOptions) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let episodes = EpisodeIterator::new(podcast, options);
    for episode in episodes {
        let path = formatting::format_episode(podcast, episode, &options.template)?;
        write_episode(episode, &path, &client).await?;
    }
    Ok(())
}

async fn write_episode(episode: &Episode, path: &str, client: &reqwest::Client) -> Result<(), Error> {
    let file_path = create_path(path)?;
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

fn create_path(path: &str) -> Result<PathBuf, Error> {
    let pathbuf = PathBuf::from_str(path).unwrap();
    if !pathbuf.parent().unwrap().exists() {
        std::fs::create_dir_all(pathbuf.parent().unwrap())?;
    }
    return Ok(pathbuf);
}
