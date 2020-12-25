from . import networking, podcast
from rich import print

def search(term):
    """Searches for a podcast on webstores"""
    search_results = []
    search_results += _search_itunes(term)
    return search_results

def _search_itunes(term):
    """Search for podcasts on the itunes store"""
    url = f"https://itunes.apple.com/search?media=podcast&term={term}"
    data = networking.get_json(url)
    if data == None:
        return None
    itunes_results = data["results"]
    search_results = []
    for result in itunes_results:
        p = podcast.Podcast(
            result["collectionName"],
            result["feedUrl"]
        )
        p.artist = result["artistName"]
        search_results.append(p)
    return search_results

def print_results(results):
    """Prints out search results"""
    for result in reversed(results):
        print(f"[cyan]{result.title}[/cyan]\n[white]by {result.artist}\n{result.feed_url}\n")
