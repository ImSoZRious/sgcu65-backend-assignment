import requests
import json

hostname = 'http://localhost'
port = 3030

base_url = hostname + ':' + str(port)

def get(path: str, data: object = None):
  # root
  if len(path) == 0:
    return requests.get(base_url)

  url = base_url + path if path[0] == '/' else base_url + '/' + path
  data_string = json.dumps(data)

  return requests.get(url, data=data)

def get(path: str, data: object = None):
  # Root
  if len(path) == 0:
    return requests.post(base_url)

  url = base_url + path if path[0] == '/' else base_url + '/' + path
  data_string = json.dumps(data)

  return requests.post(url, data=data_string)