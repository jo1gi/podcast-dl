from setuptools import setup, find_packages

setup(
    name="podcast-dl",
    version="0.0.0",
    packages=find_packages(),
    description="Downloads podcasts",
    install_requires=["requests", "argparse", "feedparser"],
    entry_points={
        'console_scripts': [
            'podcast-dl=podcastdl.main:run'
        ]
    },
)
