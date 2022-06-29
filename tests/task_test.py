import helper as h
import json

SUCESS = 0
FAILED = 1
CONNECTION_ERROR = 2

create_id = None

def is_task(data):
  return 'name' in data and 'content' in data and 'deadline' in data and 'status' in data and 'id' in data

def create_test():
  global create_id
  try:
    task_payload = {
      'name': 'assignment',
      'content': 'Backend assignment',
      'status': 'in progress',
      'deadline': '2022-07-02 00:00:00'
    }
    res = h.post('/task', task_payload)

    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)

    if not is_task(res_data):
      return FAILED
    
    create_id = str(res_data['id'])

    return SUCESS

  except:
    return CONNECTION_ERROR

def read_test():
  try:
    res = h.get('/task')
    if res.status_code != 200:
      return FAILED
    res_data = json.loads(res.text)
    if len(res_data) < 1:
      return FAILED
    test_data = res_data[0]
    if not is_task(test_data):
      return FAILED

    return SUCESS
  except:
    return CONNECTION_ERROR

def update_test():
  try:
    new_data = {
      'deadline': '2022-07-30 00:00:00'
    }
    res = h.put('/task/' + create_id, new_data)
    if res.status_code != 200:
      return FAILED
    return SUCESS  
  except:
    return CONNECTION_ERROR

def delete_test():
  try:
    res = h.delete('/task/' + create_id)
    if res.status_code != 200:
      return FAILED
    return SUCESS
  except:
    return CONNECTION_ERROR

def find_test():
  try:
    res = h.get('/task/' + create_id)
    if res.status_code != 200:
      return FAILED

    return SUCESS
  except:
    return CONNECTION_ERROR

def task_test():
  tests_name = ['Create', 'Read', 'Update', 'Find', 'Delete']
  tests_func = [create_test, read_test, update_test, find_test, delete_test]

  eval_func = {0: h.log_success, 1: h.log_failure, 2: h.log_connection_error}

  for test_name, test_func in zip(tests_name, tests_func):
    if test_name == 'Create' or create_id is not None:
      eval_func[test_func()](test_name)