import requests as r

hostname = 'http://localhost'
port = 3030

base_url = hostname + ':' + str(port)

def get(path: str, data: object = None):
  # Root path
  if not isinstance(path, str):
    raise TypeError("Except path as first argument")

  if len(path) == 0:
    return r.get(base_url)

  url = base_url + path if path[0] == '/' else base_url + '/' + path

  return r.get(url)

def main():
  k = get('/hello/warp')

  print(k.text)

if __name__ == '__main__':
  main()