use crate::{Podcast, Episode};
use crate::error::ParseError;

pub fn parse_rss_feed(content: &bytes::Bytes) -> Result<Podcast, ParseError> {
    let channel = rss::Channel::read_from(&content[..])?;
    Ok(Podcast {
        title: channel.title.clone(),
        description: Some(channel.description.clone()),
        episodes: extract_episodes(&channel)?,
    })
}

fn extract_episodes(channel: &rss::Channel) -> Result<Vec<Episode>, ParseError> {
    channel.items.iter()
        .rev()
        .enumerate()
        .map(|(index, item)| Ok(Episode {
            title: item.title.clone().unwrap(),
            link: item.enclosure.clone().unwrap().url,
            pub_date: item.pub_date.as_ref()
                .and_then(|x| chrono::DateTime::parse_from_rfc2822(&x).ok()),
            index: Some(index+1),
            author: item.author.clone(),
        }))
        .collect()
}
