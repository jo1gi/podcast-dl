from . import arguments, search, download

def run():
    try:
        options = arguments.parse_args()
        if not options.search == None:
            search_results = search.search(options.search)
            if search_results == None:
                print("Could not find any podcasts")
                exit()
            search.print_results(search_results)
        if not options.url == None:
            download.download(
                options.url,
                limit=options.limit,
                oldest=not options.newest,
                full_title=option.full_title,
                overwrite=options.overwrite,
            )
    except KeyboardInterrupt:
        pass

if __name__ == "__main__":
    run()
