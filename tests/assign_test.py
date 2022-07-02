import helper as h
import json

SUCCESS = 0
FAILED = 1
CONNECTION_ERROR = 2

user_id1 = None
task_id1 = None
task_id2 = None
team_id1 = None

def is_user(data):
  return 'id' in data and 'email' in data and 'firstname' in data and 'lastname' in data and 'role' in data

def is_task(data):
  return 'name' in data and 'content' in data and 'deadline' in data and 'status' in data and 'id' in data

def is_team(data):
  return 'name' in data

def create_user():
  global user_id1
  try:
    user_payload = {
      'email': 'abc@gmail.com',
      'firstname': 'Finding',
      'lastname': 'Nemo',
      'role': 'UX/UI Designer',
      'pwd': '123456'
    }
    res = h.post('/user', user_payload)

    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)

    if not is_user(res_data):
      return FAILED
    
    user_id1 = str(res_data['id'])

    return SUCCESS

  except:
    return CONNECTION_ERROR

def create_task():
  global task_id1, task_id2
  try:
    task = [
    {
      'name': 'Comprog',
      'content': 'Grader',
      'status': 'Done',
      'deadline': '2022-10-30 15:00:00'
    },
    {
      'name': 'Calculus',
      'content': 'Homework',
      'status': 'In progress',
      'deadline': '2022-12-15 12:00:33'
    }]
    res = h.post('/task', task[0])

    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)

    if not is_task(res_data):
      return FAILED
    
    task_id1 = str(res_data['id'])
    res = h.post('/task', task[1])

    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)

    if not is_task(res_data):
      return FAILED
    
    task_id2 = str(res_data['id'])

    return SUCCESS

  except:
    return CONNECTION_ERROR

def create_team():
  global team_id1
  try:
    team = {
      'name': 'Let\'s game it out'
    }
    res = h.post('/team', team)

    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)

    if not is_team(res_data):
      return FAILED
    
    team_id1 = str(res_data['id'])
    return SUCCESS

  except:
    return CONNECTION_ERROR


def assign_user():
  try:
    assign_obj_1 = {
      'team_id': int(team_id1),
      'user_id': int(user_id1)
    }

    res = h.post('/assign_user', assign_obj_1)
    if res.status_code != 200:
      return FAILED
    
    return SUCCESS

  except:
    return CONNECTION_ERROR


def accept_task():
  try:
    assign_obj_1 = {
      'team_id': int(team_id1),
      'task_id': int(task_id1)
    }
    assign_obj_2 = {
      'team_id': int(team_id1),
      'task_id': int(task_id2)
    }

    res = h.post('/accept_task', assign_obj_1)
    if res.status_code != 200:
      return FAILED
    
    res = h.post('/accept_task', assign_obj_2)
    if res.status_code != 200:
      return FAILED
    
    return SUCCESS

  except:
    return CONNECTION_ERROR

def get_team_from_task():
  try:
    res = h.get("task/{}/team".format(task_id1))
    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)
    
    if not is_team(res_data) or str(res_data['id']) != team_id1:
      return FAILED

    res = h.get("task/{}/team".format(task_id2))
    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)
    
    if not is_team(res_data) or str(res_data['id']) != team_id1:
      return FAILED
    return SUCCESS

  except:
    return CONNECTION_ERROR

def get_task_from_team():
  try:
    res = h.get("team/{}/task".format(team_id1))
    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)
    if len(res_data) != 2 or not all([is_task(x) for x in res_data]):
      return FAILED
    return SUCCESS

  except:
    return CONNECTION_ERROR

def get_team_from_user():
  try:
    res = h.get("user/{}/team".format(user_id1))
    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)
    if not is_team(res_data):
      return FAILED
    return SUCCESS

  except:
    return CONNECTION_ERROR


def get_user_from_team():
  try:
    res = h.get("team/{}/user".format(team_id1))
    if res.status_code != 200:
      return FAILED

    res_data = json.loads(res.text)

    if len(res_data) != 1 or not is_user(res_data[0]):
      return FAILED

    return SUCCESS

  except:
    return CONNECTION_ERROR

def clean_up():
  try:
    res = h.delete("user/{}".format(user_id1))
    if res.status_code != 200:
      return FAILED
    
    res = h.delete("task/{}".format(task_id1))
    if res.status_code != 200:
      return FAILED
    
    res = h.delete("task/{}".format(task_id2))
    if res.status_code != 200:
      return FAILED
    
    res = h.delete("team/{}".format(team_id1))
    if res.status_code != 200:
      return FAILED
    
    return SUCCESS
  except:
    return CONNECTION_ERROR

def assign_test():
  tests_name = ['Create User', 'Create Team', 'Create Task', 'Assign User', 'Accept Task', 'Check team from task', 'Check task from team', 'Check user from team', 'Check team from user', 'Cleanup']
  tests_func = [create_user, create_team, create_task, assign_user, accept_task, get_team_from_task, get_task_from_team, get_user_from_team, get_team_from_user, clean_up]

  eval_func = {0: h.log_success, 1: h.log_failure, 2: h.log_connection_error}

  for test_name, test_func in zip(tests_name, tests_func):
    if 'Create' in test_name or all([x is not None for x in [user_id1, task_id1, task_id2]]):
      eval_func[test_func()](test_name)