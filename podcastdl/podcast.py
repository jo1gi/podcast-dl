class Podcast:
    """Stores information about a podcast"""

    description = None
    artist = None
    _episodes = []

    def __init__(self, title, feed_url):
        self.title = title
        self.feed_url = feed_url

    def add_episode(self, episode):
        self._episodes.append(episode)

    def __len__(self):
        return len(self._episodes)

    def __iter__(self):
        return self._episodes.__iter__()

class Episode:

    def __init__(self, title, url):
        self.title = title
        self.url = url

