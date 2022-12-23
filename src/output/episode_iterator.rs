use crate::{Podcast, Episode};
use super::WriteOptions;

pub struct EpisodeIterator<'a> {
    index: usize,
    limit: Option<usize>,
    reversed: bool,
    items: &'a Vec<Episode>,
}

impl<'a> EpisodeIterator<'a> {

    pub fn new(podcast: &'a Podcast, options: &WriteOptions) -> Self {
        Self {
            index: options.offset.unwrap_or(0),
            limit: options.limit.clone(),
            reversed: !options.oldest,
            items: &podcast.episodes,
        }
    }

    /// Gets current index based on `self.index` and `self.reversed`
    fn get_index(&self) -> usize {
        if self.reversed {
            self.items.len() - 1 - self.index
        } else {
            self.index
        }
    }

    /// Checks if `self.index` as reached `self.limit` or has reached end of `self.items`
    fn reached_limit(&self) -> bool {
        if let Some(limit) = self.limit {
            if self.index == limit {
                return true;
            }
        }
        if self.index == self.items.len() {
            return true;
        }
        return false;
    }

}

impl<'a> Iterator for EpisodeIterator<'a> {
    type Item = &'a Episode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_limit() {
            return None;
        }
        let index = self.get_index();
        self.index += 1;
        return self.items.get(index);
    }
}
