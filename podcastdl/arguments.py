import argparse

def parse_args():
    parser = argparse.ArgumentParser(description="Downloads podcasts")
    parser.add_argument(
        '-s',
        '--search',
        help="Search for a podcast",
    )
    parser.add_argument(
        '-u',
        '--url',
        dest="url",
        help="Url of podcast to download",
    )
    parser.add_argument(
        '-l',
        '--limit',
        dest='limit',
        help="Maximum number of episodes to download",
        default=None,
        type=int,
    )
    parser.add_argument(
        '--newest',
        dest='newest',
        help="Download the newest episodes first instead of the oldest",
        action='store_true',
    )
    parser.add_argument(
        '--full-title',
        dest='full_title',
        help="Show full title of episodes in output",
        action='store_true',
    )
    parser.add_argument(
        '--overwrite',
        dest='overwrite',
        help="Overwrite already existing episode files",
        action='store_true'
    )
    return parser.parse_args()
