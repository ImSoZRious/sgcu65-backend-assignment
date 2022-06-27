import requests
import json

hostname = 'http://localhost'
port = 3030

base_url = hostname + ':' + str(port)

def get(path: str, data: object = None):
  data_string = json.dumps(data)

  # Root
  if len(path) == 0:
    return requests.get(base_url, data=data_string)

  url = base_url + path if path[0] == '/' else base_url + '/' + path

  return requests.get(url, data=data_string)

def post(path: str, data: object = None):
  data_string = json.dumps(data)

  # Root
  if len(path) == 0:
    return requests.post(base_url, data=data_string)

  url = base_url + path if path[0] == '/' else base_url + '/' + path

  return requests.post(url, data=data_string)

def post(path: str, data: object = None):
  data_string = json.dumps(data)

  # Root
  if len(path) == 0:
    return requests.put(base_url, data=data_string)

  url = base_url + path if path[0] == '/' else base_url + '/' + path

  return requests.put(url, data=data_string)

def post(path: str, data: object = None):
  data_string = json.dumps(data)

  # Root
  if len(path) == 0:
    return requests.delete(base_url, data=data_string)

  url = base_url + path if path[0] == '/' else base_url + '/' + path

  return requests.delete(url, data=data_string)

def log_success(*msg):
  print('✔️:', *msg)

def log_failure(*msg):
  print('❌:', *msg)