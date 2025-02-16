import helper as h
import json

SUCCESS = 0
FAILED = 1
CONNECTION_ERROR = 2

create_id = None

def is_user(data):
  return 'id' in data and 'email' in data and 'firstname' in data and 'lastname' in data and 'role' in data

def create_test():
  global create_id
  try:
    user_payload = {
      'email': 'abc@gmail.com',
      'firstname': 'Finding',
      'lastname': 'Nemo',
      'role': 'UX/UI Designer',
      'pwd': 'Hahaha'
    }
    res = h.post('/user', user_payload)

    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)

    if not is_user(res_data):
      return FAILED
    
    create_id = str(res_data['id'])

    return SUCCESS

  except:
    return CONNECTION_ERROR

def read_test():
  try:
    res = h.get('/user')
    if res.status_code != 200:
      return FAILED
    res_data = json.loads(res.text)
    if len(res_data) < 1:
      return FAILED
    test_data = res_data[0]
    if not is_user(test_data):
      return FAILED

    return SUCCESS
  except:
    return CONNECTION_ERROR

def update_test():
  try:
    new_data = {
      'role': 'Backend Developer'
    }
    res = h.put('/user/' + create_id, new_data)
    if res.status_code != 200:
      return FAILED
    return SUCCESS  
  except:
    return CONNECTION_ERROR

def delete_test():
  try:
    res = h.delete('/user/' + create_id)
    if res.status_code != 200:
      return FAILED
    return SUCCESS
  except:
    return CONNECTION_ERROR

def find_test():
  try:
    res = h.get('/user/' + create_id)
    if res.status_code != 200:
      return FAILED
    res_data = json.loads(res.text)
    if not is_user(res_data):
      return FAILED

    return SUCCESS
  except:
    return CONNECTION_ERROR

def query_test():
  try:
    firstname = "Fin"
    lastname = "N"
    url1 = '/user/search?firstname={}'.format(firstname)
    url2 = '/user/search?lastname={}'.format(lastname)
    url3 = '/user/search?firstname={}&lastname={}'.format(firstname, lastname)

    for url in [url1, url2, url3]:
      res = h.get(url)
      if res.status_code != 200:
        return FAILED
      res_data = json.loads(res.text)
      if len(res_data) == 0:
        continue
      res_data = res_data[0]
      if not is_user(res_data):
        return FAILED

    return SUCCESS
  except:
    return CONNECTION_ERROR

def user_test():
  tests_name = ['Create', 'Read', 'Update', 'Find', 'Query', 'Delete']
  tests_func = [create_test, read_test, update_test, find_test, query_test, delete_test]

  eval_func = {0: h.log_success, 1: h.log_failure, 2: h.log_connection_error}

  for test_name, test_func in zip(tests_name, tests_func):
    if test_name == 'Create' or create_id is not None:
      eval_func[test_func()](test_name)