from .podcast import Podcast, Episode
from rich.progress import Progress
from rich import print
import time, os, feedparser, requests

def download(url, **kwargs):
    """Downloads a podcast"""
    podcast = _get_feed(url, **kwargs)
    _download_episodes(podcast)

def _get_feed(url, **kwargs):
    feed = feedparser.parse(url)
    podcast = Podcast(feed.feed.title, url)
    entries = _get_audio_entries(feed.entries)
    entries = _filter_entries(entries, **kwargs)
    for episode in _entries_to_episodes(entries):
        podcast.add_episode(episode)
    return podcast

def _download_episode(episode, feed_title, overwrite):
    """Downloads a single episode"""
    url = episode.url
    req = requests.get(url, stream=True)
    output_file = os.path.join(feed_title, f"{episode.title}.mp3")
    if os.path.isfile(output_file):
        if overwrite or os.path.getsize(output_file) < int(req.headers['Content-length']):
            os.remove(output_file)
        else:
            print(f"[blue]{episode.title}[/blue] already exists. skipping")
            return
    with Progress() as progress:
        task = progress.add_task(f"Downloading [blue]{episode.title}[/blue]", total=int(req.headers['Content-length']))
        with open(output_file, "ab") as f:
            for chunk in req.iter_content(chunk_size=1024):
                f.write(chunk)
                progress.update(task, advance=1024)

def _download_episodes(podcast, overwrite=False):
    """Downloads episodes from a list"""
    print(f"Downloading [red]{len(podcast)}[/red] episodes from [cyan]{podcast.title}[/cyan]")
    if not os.path.isdir(podcast.title):
        os.mkdir(podcast.title)
    for i in podcast:
        _download_episode(i, podcast.title, overwrite)

def _filter_entries(entries, limit=None, oldest=True, **kwargs):
    if oldest:
        entries = [entry for entry in reversed(entries)]
    if type(limit) == int:
        entries = entries[:limit]
    return entries

def _entries_to_episodes(entries):
    """Converts a list of entries to a list of episodes"""
    episodes = []
    for entry in entries:
        episodes.append(_entry_to_episode(entry))
    return episodes

def _entry_to_episode(entry):
    """Converts an entry to an episode"""
    episode = Episode(entry.title, entry["links"][0]["href"])
    return episode

def _get_audio_entries(entries):
    """Finds all entries with a link to an audio file"""
    audio_entries = []
    for entry in entries:
        is_audio = False
        remove = []
        for n, link in enumerate(entry["links"]):
            if _is_audio_link(link):
                is_audio = True
            else:
                remove.append(n)
        if is_audio:
            for i in remove:
                del entry["links"][i]
            audio_entries.append(entry)
    return audio_entries

def _is_audio_link(link):
    """Checks if a given link is an audio file"""
    if "type" in link and link["type"][:5] == "audio":
        return True
    if link["href"].endswith(".mp3"):
        return True
    return False
