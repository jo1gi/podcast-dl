import requests, json

def get(url, status_code=None, **kwargs):
    """Downloads data from the given url"""
    try:
        resp = requests.get(url, **kwargs)
        if not status_code == None:
            if not resp.status_code == status_code:
                return None
        return resp.content
    except Exception as e:
        return None

def get_json(url, **kwargs):
    """Downloads json data and converts it to a dict"""
    raw = get(url, **kwargs)
    if raw == None:
        return None
    return json.loads(raw.decode('utf8'))
