import helper as h
import json

SUCESS = 0
FAILED = 1
CONNECTION_ERROR = 2

def is_user(data):
  return 'id' in data and 'email' in data and 'firstname' in data and 'lastname' in data and 'role' in data

def create_test():
  try:
    user_payload = {
      'email': 'abc@gmail.com',
      'firstname': 'Finding',
      'lastname': 'Nemo',
      'role': 'UX/UI Designer'
    }
    res = h.post('/user', user_payload)

    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)

    if not is_user(res_data):
      return FAILED
    
    return SUCESS

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

    return SUCESS
  except:
    return CONNECTION_ERROR

def update_test():
  try:
    new_data = {
      'role': 'Backend Developer'
    }
    res = h.put('/user/10001', new_data)
    if res.status_code != 200:
      return FAILED
    res_data = json.loads(res.text)
    if not (is_user(res_data) and res['role'] == 'Backend Developer'):
      return FAILED

    return SUCESS  
  except:
    return CONNECTION_ERROR

def delete_test():
  try:
    res = h.delete('/user/10001')
    if res.status_code != 200:
      return FAILED
    res_data = json.loads(res.text)
    if not is_user(res_data):
      return FAILED
    return SUCESS
  except:
    return CONNECTION_ERROR

def find_test():
  try:
    res = h.get('/user/10001')
    if res.status_code != 200:
      return FAILED
    res_data = json.loads(res.text)
    if not is_user(res_data):
      return FAILED

    return SUCESS
  except:
    return CONNECTION_ERROR

def user_test():
  tests_name = ['Create', 'Read', 'Update', 'Delete', 'Find']
  tests_func = [create_test, read_test, update_test, delete_test, find_test]

  eval_func = {0: h.log_success, 1: h.log_failure, 2: h.log_connection_error}

  for test_name, test_func in zip(tests_name, tests_func):
    eval_func[test_func()](test_name)